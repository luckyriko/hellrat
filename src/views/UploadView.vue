<template>
  <div>
    <div ref="drop" class="area">
      <div class="env">{{ $t('upload.envName', { name: environment.name }) }}</div>
      <div class="title">{{ $t('upload.tip1') }}</div>
      <div class="title">{{ $t('upload.tip2') }}</div>
      <div class="title">{{ $t('upload.tip3') }}</div>

      <div class="bb">
        <el-button type="primary" @click="selectDirs">{{ $t('upload.selectDirs') }}</el-button>
        <el-button type="primary" @click="selectFiles">{{ $t('upload.selectFiles') }}</el-button>
      </div>
    </div>
    <div>
      <div class="box" v-for="(item, index) in files" :key="index">
        <div class="left">{{ index + 1 + '. ' + item.name }}</div>
        <div class="right" v-if="item.status != 'non-support' && item.status != 're-path' && item.status != 'error'">
          {{ getStatusName(item.status) }}
        </div>
        <div class="right error" v-if="item.status == 'non-support' || item.status == 'error'">
          {{ getStatusName(item.status) }}
        </div>
        <div class="right warning" v-if="item.status == 're-path'">
          {{ getStatusName(item.status) }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { basename } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const files = ref([]);

const environment = ref({
  id: 1,
  name: 'default',
  activate: 1
});

const getStatusName = (status) => {
  let statusName = t('upload.unknow');
  switch (status) {
    case 'wait':
      statusName = t('upload.wait');
      break;
    case 'start':
      statusName = t('upload.start');
      break;
    case 're-path':
      statusName = t('upload.re-path');
      break;
    case 'copy':
      statusName = t('upload.copy');
      break;
    case 'unzip':
      statusName = t('upload.unzip');
      break;
    case 'non-support':
      statusName = t('upload.nonSupport');
      break;
    case 'save':
      statusName = t('upload.save');
      break;
    case 'end':
      statusName = t('upload.end');
      break;
    case 'error':
      statusName = t('upload.error');
      break;
    default:
  }
  return statusName;
};

const selectFiles = async () => {
  const selectedPath = await open({
    multiple: true,
    directory: false,
    title: t('upload.selectFilesTip'),
  });

  if (selectedPath) {
    receiveModsFilePath(selectedPath)
  }

}

const selectDirs = async () => {
  const selectedPath = await open({
    multiple: true,
    directory: true,
    title: t('upload.selectDirsTip'),
  });

  if (selectedPath) {
    receiveModsFilePath(selectedPath)
  }

}

// 获取当前环境
async function getEnvironmentList() {
  try {
    let env = await invoke('get_environment_list');
    let activateItem = env.find(x => x.activate == 1)
    environment.value = activateItem;

  } catch (error) {
    console.error('获取当前环境变量失败:', error);
    ElMessage.error(t('upload.getEnvironmentFail') + error)

  }
}

async function receiveModsFilePath(paths) {
  if (!paths) {
    ElMessage.error(t('upload.getPathFail'))
    return;
  }
  try {
    files.value = [];
    await Promise.allSettled(paths.map(async element => {
      files.value.push({
        name: await basename(element),
        status: 'wait'
      })
    }));

    await invoke('save_mods', { paths, env_id: Number(environment.value.id) });
    console.log('save_mods successfully!');
  } catch (error) {
    console.error('save_mods fail:', error);
    ElMessage({
      showClose: true,
      message: t('upload.saveModError') + error,
      type: 'error',
      duration: 0
    })

  }
}
let dragDropUnlisten = null;
let eventUnlisten = null;


onMounted(async () => {
  dragDropUnlisten = await getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'over') {
      // console.log('User hovering', event.payload.position);
    } else if (event.payload.type === 'drop') {
      console.log('User dropped', event.payload.paths);
      files.value = [];

      let paths = event.payload.paths;
      receiveModsFilePath(paths)
    } else {
      console.log('File drop cancelled');
    }
  });

  await getEnvironmentList();


  eventUnlisten = await listen('save-mods-progress', (event) => {
    console.log(event.payload);
    try {
      let progress = JSON.parse(event.payload);
      if (progress) {
        files.value[progress.index]['status'] = progress.status;
      }
    } catch (e) {
      console.log(e);
    }
  });


})
onBeforeUnmount(() => {
  dragDropUnlisten();
  eventUnlisten();
})


</script>

<style>
.area {
  /* height: 200px; */
  width: 100%;
  background-color: #ececec;
  padding: 20px;
  box-sizing: border-box;

  .title {
    width: 100%;
    font-weight: bold;
    color: #666;
    text-align: center;
  }

  .env {
    width: 100%;
    color: red;
    font-weight: bold;
    text-align: left;
    font-size: 12px;
    margin-top: -10px;
    margin-left: -10px;
  }

}

.bb {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  margin-top: 20px;
}

.box {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 0 10px;
  margin-top: 5px;
  font-size: 12px;
  box-sizing: border-box;

  .left {
    flex: 1;
  }

  .right {
    width: 100px;
    text-align: center;
  }
}

.error {
  color: red;
}

.warning {
  color: orange;
}
</style>