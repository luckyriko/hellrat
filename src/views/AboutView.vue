<template>
  <div class="container">
    <el-row>
      <el-col :span="24">
        <div class="title">关于</div>
      </el-col>
    </el-row>

    <el-row>
      <el-col :span="24" class="row">
        <span style="margin-right: 20px;">版本号：{{ version }}</span>
        <el-button @click="getUpdate" type="success" plain>检查更新</el-button>

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
import { fetch } from "@tauri-apps/plugin-http";
import semver from "semver";

const version = ref("");

async function fetchAppVersion() {
  try {
    version.value = await getVersion();
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

async function getUpdate() {
  if (!version.value) {
    ElMessage.error('本地版本获取失败，无法检测更新！')
    return;
  }

  try {
    const response = await fetch("https://api.luckyriko.com/app/getLatestVersion");
    // console.log(response.status);  // e.g. 200
    // console.log(response.statusText); // e.g. "OK"

    if (response.status != 200) {
      ElMessage.error('服务器正在维护中，请稍后再试！')
      return;
    }

    const { version: latestVersion, upMsg = '暂无描述', downUrl = '' } = await response.json();
    if (semver.gt(latestVersion, version.value)) {
      console.log('有新版本发布了！');
      ElMessageBox.confirm(
        '更新内容：' + upMsg,
        '最新版本：v' + latestVersion,
        {
          confirmButtonText: '前往下载页',
          cancelButtonText: '取消',
          type: 'info',
        }
      )
        .then(async () => {
          if (downUrl) {
            await open(downUrl);
          } else {
            this.donate()
          }
        })
        .catch(() => {
        })

    } else {
      ElMessage({
        message: '已经是最新版本了',
        type: 'success',
      })
    }

  } catch (error) {
    console.error("请求失败:", error);
  }
}


</script>

<style scoped lang="scss">
.container {
  padding: 20px;
}

.title {
  font-size: 20px;
  font-weight: bold;
  margin-bottom: 10px;
}

.row {
  margin-bottom: 20px;
  /* 设置每一行与下一行的间隔 */
}
</style>
