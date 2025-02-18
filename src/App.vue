<script setup>
import { ref } from "vue";
import { RouterLink, RouterView } from "vue-router";
import HelloWorld from "./components/HelloWorld.vue";
// import { invoke } from "@tauri-apps/api/core";
// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
</script>

<template>
  <div class="page">
    <div class="left">
      <header>
        <div class="logo-text">Mods管理器</div>
        <!-- <img alt="app logo" class="logo" src="@src/assets/icon.png" width="100" height="100" /> -->
      </header>
      <nav>
        <RouterLink to="/" class="nav-link" active-class="active">列表</RouterLink>
        <RouterLink to="/add" class="nav-link" active-class="active">安装</RouterLink>
        <!-- <RouterLink to="/import" class="nav-link">导入</RouterLink> -->
        <RouterLink to="/setting" class="nav-link" active-class="active">设置</RouterLink>
        <RouterLink to="/about" class="nav-link" active-class="active">关于</RouterLink>
      </nav>
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

    .logo-text{
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
  }

  .right {
    width: 100%;
    height: 100vh;
  }
}
</style>
