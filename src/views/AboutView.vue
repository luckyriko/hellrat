<template>
  <div class="container">
    <el-row>
      <el-col :span="24">
        <div class="title">关于</div>
      </el-col>
    </el-row>

    <el-row>
      <el-col :span="24">
        版本号：{{version}}
      </el-col>
      <el-col :span="24">
        作者：理（luckyriko@qq.com）
      </el-col>
      <el-col :span="24">
        本软件免费，禁止商业用途。
      </el-col>
      <el-col :span="24">
        本软件目前处于开发阶段，已经测试过但不保证功能稳定性。
      </el-col>
      <el-col :span="24">
        本软件使用Tauri开发，请使用Win10、Win11，不保证也没有测试过Win7、Win8可用性。
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { getVersion } from '@tauri-apps/api/app';
import { invoke } from '@tauri-apps/api/core';
const greetMsg = ref("");
const version = ref("");

async function fetchAppVersion() {
  try {
    version.value = await getVersion();
    console.log('App Version:', version);
  } catch (error) {
    console.error('Failed to get app version:', error);
  }
}

fetchAppVersion();
</script>

<style scoped lang="scss">
.container {
  padding: 30px;
}

.title {
  font-size: 20px;
  font-weight: bold;
  margin-bottom: 10px;
}
</style>
