<template>
  <div class="container">
    <div v-for="(value, index) in chat" :key="index" class="grid-item" @click="confirmInput(value)">
      {{ value }}
    </div>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { ref, reactive, onMounted } from 'vue'
import { Store } from '@tauri-apps/plugin-store'

const chat = reactive({
  chat1: '你好！',
  chat2: '再见！',
  chat3: 'Ciallo～(∠・ω< )⌒☆',
  chat4: '',
  chat5: '',
  chat6: '',
  chat7: '',
  chat8: '',
  chat9: '',
})

onMounted(async () => {
  const loading = ElLoading.service({
    lock: true,
    text: 'Loading',
    background: 'rgba(0, 0, 0, 0.7)',
  })
  try {
    const store = await Store.load('config.json', { autoSave: false })

    const storeQuicklyChat = await store.get('quickly_chat');
    console.log(storeQuicklyChat);

    if (storeQuicklyChat && storeQuicklyChat['chat']) {
      for (const [key, value] of Object.entries(chat)) {
        chat[key] = storeQuicklyChat['chat'][key] || value;
      }
    }

  } catch (error) {
    console.error('获取配置文件失败:', error);
    ElMessage.error(error)

  } finally {
    loading.close();

  }

});


async function confirmInput(text) {
  if (!text) {
    await getCurrentWebviewWindow().hide();
    return;
  }
  await invoke('confirm_input', { text, label: 'quickly-chat' });
}

</script>

<style>
.container {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  width: 100%;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);

}

.grid-item {
  display: flex;
  justify-content: center;
  align-items: center;
  border: 1px solid white;
  font-size: 24px;
  color: white;

}
</style>