<template>
  <div>
    <div ref="drop" class="area">
      <div class="title">拖动一个或多个</div>
      <div class="title">目录 / 7z、zip、rar压缩文件</div>
      <div class="title">至此页面任意位置进行上传，也可手动选择</div>

      <div class="bb">
        <el-button type="primary" @click="selectDirs">选择目录</el-button>
        <el-button type="primary" @click="selectFiles">选择文件</el-button>
      </div>
    </div>
    <div>
      <div class="box" v-for="(item, index) in files" :key="index">
        <div class="left">{{ index + 1 + '. ' + item.name }}</div>
        <div class="right" :class="{ 'error': (item.status == 'non-support' || item.status == 're-path') }">
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

const files = ref([]);

const environment = ref({
  id: 1,
  name: 'default',
  activate: 1
});

const getStatusName = (status) => {
  let statusName = '未知状态';
  switch (status) {
    case 'wait':
      statusName = '等待ing';
      break;
    case 'start':
      statusName = '已开始';
      break;
    case 're-path':
      statusName = '重复保存-跳过';
      break;
    case 'copy':
      statusName = '复制ing';
      break;
    case 'unzip':
      statusName = '解压ing';
      break;
    case 'non-support':
      statusName = '不支持的文件格式';
      break;
    case 'save':
      statusName = '保存ing';
    case 'end':
      statusName = '已完成';
      break;
    default:
  }
  return statusName;
};

const selectFiles = async () => {
  const selectedPath = await open({
    multiple: true,
    directory: false,
    title: "请选择至少一个Mod压缩文件",
  });

  if (selectedPath) {
    receiveModsFilePath(selectedPath)
  }

}

const selectDirs = async () => {
  const selectedPath = await open({
    multiple: true,
    directory: true,
    title: "请选择至少一个Mod目录",
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
    ElMessage.error(error || 'Oops, this is a error message.')

  }
}

async function receiveModsFilePath(paths) {
  if (!paths) {
    ElMessage.error('没有获取到文件路径')
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
    ElMessage.error(error || '存档失败');
    files.value = [];

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
</style>