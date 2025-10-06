<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">{{ $t('about.title') }}</div>
        </el-col>
      </el-row>

      <el-row>
        <el-col :span="24" class="row">
          <span style="margin-right: 20px;">{{ $t('about.version', [version]) }}</span>
          <el-button @click="getUpdate" type="success" plain>{{ $t('about.checkUpdate') }}</el-button>

        </el-col>
        <el-col :span="24" class="row">
          {{ $t('about.author') + '理 (luckyriko@qq.com)' }}
        </el-col>
        <el-col :span="24" class="row">
          {{ $t('about.qqGroupChatNo') + '670106990' }}
        </el-col>
        <el-col :span="24" class="row">
          <span style="margin-right: 5px;">{{ $t('about.video') }}</span>
          <el-link :underline="true" type="primary" @click="suggest()">bilibili</el-link>
        </el-col>
        <el-col :span="24" class="row">
          <span style="margin-right: 5px;">{{ $t('about.versionInfo') }}</span>
          <el-link :underline="true" type="primary" @click="donate()">
            {{ $t('about.afdian') }}
          </el-link>
        </el-col>

        <h4>{{ $t('about.userAgreement.title') }}</h4>
        <el-col :span="24" class="row">
          {{ $t('about.userAgreement.detail') }} </el-col>


        <h4>{{ $t('about.disclaimer.title') }}</h4>
        <el-col :span="24" class="row">
          {{ $t('about.disclaimer.no1') }} </el-col>
        <el-col :span="24" class="row">
          {{ $t('about.disclaimer.no2') }} </el-col>
        <el-col :span="24" class="row">
          {{ $t('about.disclaimer.no3') }} </el-col>
        <el-col :span="24" class="row">
          {{ $t('about.disclaimer.no4') }} </el-col>
        <el-col :span="24" class="row">
          {{ $t('about.disclaimer.no5') }} </el-col>

      </el-row>
    </div>
  </el-scrollbar>

</template>

<script setup>
import { ref } from "vue";
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/plugin-shell';
import { fetch } from '@tauri-apps/plugin-http';
import semver from "semver";
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

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
  await open("https://www.bilibili.com/video/BV1xze5zQEbP");

}

async function getUpdate() {
  if (!version.value) {
    ElMessage.error(t('about.getUpdate.getLocationAppVersionError'))
    return;
  }

  try {
    const response = await fetch('https://hellrat.luckyriko.com/app/getLatestVersion', {
      method: 'GET',
    });

    // console.log(response.status);  // e.g. 200
    // console.log(response.statusText); // e.g. "OK"

    if (response.status != 200) {
      ElMessage.error(t('about.serviceError'))
      return;
    }

    const { version: latestVersion, upMsg = t('about.getUpdate.noDesc'), downUrl = '' } = await response.json();
    if (semver.gt(latestVersion, version.value)) {
      console.log('有新版本发布了！');
      ElMessageBox.confirm(
        t('about.getUpdate.updateDetail') + upMsg,
        t('about.getUpdate.latestVersion') + latestVersion,
        {
          confirmButtonText: t('about.getUpdate.goDownload'),
          cancelButtonText: t('about.getUpdate.cancel'),
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
        message: t('about.getUpdate.noUpdates'),
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
  display: flex;
  align-items: center;
}
</style>
