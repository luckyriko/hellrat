<script setup>
import { ref, onMounted } from "vue";
import { RouterLink, RouterView } from "vue-router";
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/plugin-shell';
import { fetch } from "@tauri-apps/plugin-http";
import semver from "semver";

import HelloWorld from "./components/HelloWorld.vue";

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

    const response = await fetch("https://api.luckyriko.com/app/getLatestVersion");
    // console.log(response.status);  // e.g. 200
    // console.log(response.statusText); // e.g. "OK"
    if (response.status != 200) {
      ElMessage.error('服务器正在维护中，请稍后再试！')
      return;
    }

    const { version: latestVersion, upMsg: mm = '暂无描述', downUrl: dd = '' } = await response.json();
    if (semver.gt(latestVersion, version)) {
      console.log('有新版本发布了！');
      newVersion.value = `v${latestVersion}已发布`;
      upMsg.value = mm;
      downUrl.value = dd;
    }

  } catch (error) {
    console.error("请求失败:", error);
  }
})


const ddup = async () => {
  ElMessageBox.confirm(
    '更新内容：' + upMsg.value,
    '最新版本：' +newVersion.value,
    {
      confirmButtonText: '前往下载页',
      cancelButtonText: '取消',
      type: 'info',
    }
  )
    .then(async () => {
      if (downUrl.value) {
        await open(downUrl.value);
      } else {
        ElMessage.error('请在关于页前往爱发电下载！')

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
        <!-- <div class="logo-text">Mods管理器</div> -->
        <div class="logo-text">
          <img alt="app logo" class="logo" src="@src/assets/shushu.png" width="40" height="40" />
        </div>
      </header>
      <nav>
        <RouterLink to="/" class="nav-link" active-class="active">列表</RouterLink>
        <RouterLink to="/add" class="nav-link" active-class="active">安装</RouterLink>
        <!-- <RouterLink to="/import" class="nav-link">导入</RouterLink> -->
        <RouterLink to="/setting" class="nav-link" active-class="active">设置</RouterLink>
        <RouterLink to="/help" class="nav-link" active-class="active">帮助</RouterLink>
        <RouterLink to="/about" class="nav-link" active-class="active">关于</RouterLink>
        <RouterLink to="/test" class="nav-link" active-class="active">测试</RouterLink>

      </nav>

      <footer class="bottom" v-if="newVersion" @click="ddup">
        <div class="version">
          <el-icon size="15" color="orange"><i-ep-cold-drink /></el-icon> <span style="color: red;">新版本！</span>
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
  <!-- <footer>
    <HelloWorld msg="You did it!" />
  </footer> -->
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
      width: 100px;
      height: 40px;
      background-color: skyblue;
      text-align: center;
      line-height: 40px;
      color: purple;
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
