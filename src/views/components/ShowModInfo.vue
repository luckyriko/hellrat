<template>
  <!-- el-dialog 组件，使用 v-model 绑定 props.visible 以实现双向数据绑定 -->
  <el-dialog v-model="showFlag" :title="$t('showModInfo.title')" @close="handleClose" width="50%" top="5vh">
    <el-scrollbar max-height="78vh">
      <h3 style="margin-top: 0;">{{ $t('showModInfo.base') }}</h3>
      <div style="background-color: #f0f0f0;">
        <el-image :src="image" fit="cover" style="width: 100%;" v-if="image" />
      </div>
      <div class="box" v-for="(value, key, index) in mod" :key="index">
        <div style="width: 130px;margin-bottom: 5px;">{{ nameValue[key] }}</div>
        <div>{{ value }}</div>
      </div>

      <h3>{{ $t('showModInfo.installFiles') }}</h3>
      <div class="box" v-for="(value, index) in files" :key="index" v-if="files.length > 0">
        <div>{{ value }}</div>
      </div>
      <div v-else>{{ $t('showModInfo.noInstallFiles') }}</div>

      <!-- 底部插槽，提供关闭按钮 -->
      <template #footer>
        <el-button @click="handleClose">{{ $t('showModInfo.close') }}</el-button>
      </template>
    </el-scrollbar>

  </el-dialog>
</template>

<script setup>
import { ref, onUpdated } from 'vue';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { join } from '@tauri-apps/api/path';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const props = defineProps(['recordId', 'envId', 'modsStorePath']);
const showFlag = defineModel();

const mod = ref({});
const files = ref([]);
const image = ref('');

const nameValue = {
  id: 'ID',
  uuid: 'UUID',
  name: t('showModInfo.name'),
  path: t('showModInfo.path'),
  tag: t('showModInfo.tag'),
  author: t('showModInfo.author'),
  link: t('showModInfo.link'),
  desc: t('showModInfo.desc'),
  icon: t('showModInfo.icon'),
  sort: t('showModInfo.sort'),
  type: t('showModInfo.type'),
  version: t('showModInfo.version'),
  options: t('showModInfo.options'),
  activate: t('showModInfo.activate'),
  env_mod_id: t('showModInfo.envModId'),
  env_mod_options: t('showModInfo.envModOptions'),
  env_mod_install_flag: t('showModInfo.envModInstallFlag')
}

onUpdated(() => {
  console.log('ShowModInfo onUpdated', props);
  if (props.recordId && props.envId && showFlag.value) {
    getModDetail()
  }
})

async function getModDetail() {
  try {
    console.log("env_id, record_id:", props.envId, props.recordId);

    let modInfo = await invoke("get_env_mod_info_with_install_files", { env_id: Number(props.envId), record_id: props.recordId });

    if (modInfo.info) {
      delete modInfo.info.id;
      delete modInfo.info.env_mod_id;
      delete modInfo.info.type;
      delete modInfo.info.options;
      delete modInfo.info.env_mod_options;
      delete modInfo.info.sort;

      image.value = modInfo?.info?.icon ? await getPreviewPath(modInfo.info.path, modInfo.info.icon) : '';
      mod.value = modInfo.info;

    }
    files.value = modInfo.files || [];

    console.log("get_env_mod_info_with_install_files:", modInfo);
  } catch (error) {
    console.error("get_env_mod_info_with_install_files err:", error);
    ElMessage.error(t('showModInfo.fail') + String(error))

  } finally {

  }

}


const getPreviewPath = async (path, icon) => {
  if (props.modsStorePath && path && icon) {
    let imgPath = await join(props.modsStorePath, path, '/' + icon);
    const assetUrl = convertFileSrc(imgPath);
    console.log(imgPath, assetUrl);
    return assetUrl;
  } else {
    return '';
  }
}


// 定义事件，用于向父组件传递更新和关闭操作
const emit = defineEmits(['close']);

// 关闭对话框时触发的函数
const handleClose = () => {
  // 通过 update:visible 事件通知父组件更新 dialogVisible 状态
  // emit('update:visible', false);
  // 触发自定义 close 事件，父组件可以监听此事件执行额外逻辑
  emit('close');
};
</script>

<style scoped>
.box {
  display: flex;
  flex-direction: row;
  margin-left: 20px;
  align-items: center;
  margin-top: 10px;
}
</style>
