<template>
  <div class="container">
    <el-row>
      <el-col :span="24">
        <div class="title">关于</div>
      </el-col>
    </el-row>

    <el-row>
      <el-col :span="24" class="row">
        版本号：{{version}}
      </el-col>
      <el-col :span="24" class="row">
        作者：理（luckyriko@qq.com）
      </el-col>
      <el-col :span="24" class="row">
        意见反馈：<el-link :underline="true" type="primary" @click="suggest()">bilibili</el-link>
      </el-col>
      <el-col :span="24" class="row">
        版本发布&捐赠：<el-link :underline="true" type="primary" @click="donate()">爱发电</el-link>
      </el-col>
      <el-col :span="24" class="row">
        本软件免费，禁止商业用途。
      </el-col>
      <el-col :span="24" class="row">
        开发者不对因使用或无法使用本软件所造成的任何直接、间接、特殊、附带或后果性损害承担责任，包括但不限于数据丢失、业务中断或财务损失等，即使开发者已被告知可能发生此类损害。
      </el-col>
      <el-col :span="24" class="row">
        用户在使用本软件时，应自行承担使用风险。开发者不保证本软件完全无误，也不对任何错误、缺陷或功能缺失负责。用户应确保其使用本软件符合相关法律法规，并自行负责因使用本软件导致的任何后果。
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/plugin-shell';
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

const donate = async () => {
  await open("https://afdian.com/a/luckyriko");

}

const suggest = async () => {
  await open("https://www.bilibili.com/opus/1035770623780978689");

}
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

.row {
  margin-bottom: 20px; /* 设置每一行与下一行的间隔 */
}
</style>
