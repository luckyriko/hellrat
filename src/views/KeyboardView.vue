<template>
  <el-input style="height: 100vh;" :autofocus="true" v-model.lazy="text"
    size="large" placeholder="请输入文字" clearable @keydown.enter="confirmInput" autocomplete="off" :validate-event="false">
    <!-- <template #append>
      <el-button @click="confirmInput"><i-ep-promotion /></el-button>
    </template> -->
  </el-input>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { ref } from 'vue'

const text = ref('');

async function confirmInput() {
  if (!text.value) {
    await getCurrentWebviewWindow().hide();
    return;
  }
  await invoke('confirm_input', { text: text.value, label: 'keyboard' });
  text.value = '';
}

</script>

<style></style>