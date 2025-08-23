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
        QQ群：670106990
      </el-col>
      <el-col :span="24" class="row">
        视频教程：<el-link :underline="true" type="primary" @click="suggest()">bilibili</el-link>
      </el-col>
      <el-col :span="24" class="row">
        版本发布&捐赠：<el-link :underline="true" type="primary" @click="donate()">爱发电</el-link>
      </el-col>

      <h4>用户协议</h4>
      <el-col :span="24" class="row">
        本软件免费提供，禁止商业用途。
      </el-col>


      <h4>免责声明</h4>
      <el-col :span="24" class="row">
        通用性：本项目是作为一个闭源项目提供的，开发者在法律允许的范围内不对软件的功能性、安全性或适用性提供任何形式的明示或暗示的保证。
      </el-col>
      <el-col :span="24" class="row">
        无担保：用户明确理解并同意，使用本软件的风险完全由用户自己承担，软件以"现状"和"现有"基础提供。开发者不提供任何形式的担保，无论是明示还是暗示的，包括但不限于适销性、特定用途的适用性和非侵权的担保。
      </el-col>
      <el-col :span="24" class="row">
        风险承担：在任何情况下，开发者都不对任何直接的、间接的、偶然的、特殊的、惩罚性的或后果性的损害承担责任，包括但不限于使用本软件产生的数据丢失、业务中断、个人信息泄露或其他损害、损失，即使开发者已被告知可能发生此类损害。
      </el-col>
      <el-col :span="24" class="row">
        修改和版本：开发者有权在任何时间修改软件的功能或特性，以及本免责声明的任何部分，并且这些修改可能会以软件更新的形式体现。
      </el-col>
      <el-col :span="24" class="row">
        最终解释权：本免责声明的最终解释权归开发者所有。
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
  await open("https://space.bilibili.com/7387996/lists/4325372?type=season");

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
