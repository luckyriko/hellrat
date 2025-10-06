<script setup>
import { ref, onMounted } from "vue";
import { RouterLink, RouterView } from "vue-router";
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/plugin-shell';
import { fetch } from "@tauri-apps/plugin-http";
import semver from "semver";
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

// import { invoke } from "@tauri-apps/api/core";
// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

const newVersion = ref("");
const upMsg = ref("");
const downUrl = ref("");


onMounted(async () => {
  try {
    let version = await getVersion();
    if (!version) {
      return;
    }

    const response = await fetch("https://hellrat.luckyriko.com/app/getLatestVersion");
    // console.log(response.status);  // e.g. 200
    // console.log(response.statusText); // e.g. "OK"
    if (response.status != 200) {
      // ElMessage.error(t('about.serviceError'))
      return;
    }

    const { version: latestVersion, upMsg: mm = t('about.getUpdate.noDesc'), downUrl: dd = '' } = await response.json();
    if (semver.gt(latestVersion, version)) {
      console.log('有新版本发布了！');
      newVersion.value = t('about.getUpdate.latestVersion') + latestVersion;
      upMsg.value = mm;
      downUrl.value = dd;
    }

  } catch (error) {
    console.error("请求失败:", error);
  }
})


const ddup = async () => {
  ElMessageBox.confirm(
    t('about.getUpdate.updateDetail') + upMsg.value,
    t('about.getUpdate.latestVersion') + newVersion.value,
    {
      confirmButtonText: t('about.getUpdate.goDownload'),
      cancelButtonText: t('about.getUpdate.cancel'),
      type: 'info',
    }
  )
    .then(async () => {
      if (downUrl.value) {
        await open(downUrl.value);
      } else {
        ElMessage.error(t('about.getUpdate.afdianDown'))

      }
    })
    .catch(() => {
    })

}
</script>

<template>
  <div class="page">
    <div class="left">
      <header>
        <div class="logo-text">
          <img alt="HellRat" class="logo" src="/app-icon.png" width="40" height="40" />
          <span>HellRat</span>
        </div>
      </header>
      <nav>
        <RouterLink to="/home" class="nav-link" active-class="active">{{ $t('page.mods') }}</RouterLink>
        <RouterLink to="/setting" class="nav-link" active-class="active">{{ $t('page.setting') }}</RouterLink>
        <RouterLink to="/about" class="nav-link" active-class="active">{{ $t('page.about') }}</RouterLink>
        <!-- <RouterLink to="/test" class="nav-link" active-class="active">测试</RouterLink> -->

      </nav>

      <footer class="bottom" v-if="newVersion" @click="ddup">
        <div class="version">
          <el-icon size="15" color="orange"><i-ep-cold-drink /></el-icon> <span style="color: red;">
            {{ $t('page.news') }}
          </span>
        </div>
        <div class="version">{{ newVersion }}</div>
      </footer>
    </div>
    <div class="right">
      <main>
        <RouterView />
      </main>
    </div>
  </div>
</template>

<style scoped lang="scss">
.page {
  display: flex;
  flex-direction: row;

  .left {
    width: 100px;
    height: 100vh;
    background-color: #222;
    position: relative;

    .logo-text {
      box-sizing: border-box;
      padding: 2px 0;
      width: 100px;
      background-color: skyblue;
      text-align: center;
      color: #333;
      display: flex;
      flex-direction: column;
      align-items: center;
    }

    nav {
      display: flex;
      flex-direction: column;
      text-align: center;

      .nav-link {
        display: block;
        text-decoration: none;
        color: #f2f2f2;
        margin: 10px 0;
      }

      .active {
        color: red;
      }
    }

    .bottom {
      width: 100%;
      position: absolute;
      bottom: 10px;
      color: #f2f2f2;
      font-size: 14px;

      .version {
        width: 100%;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;


      }
    }
  }

  .right {
    width: 100%;
    height: 100vh;
  }
}
</style>
