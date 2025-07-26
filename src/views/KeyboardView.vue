<template>
  <el-row>
    <el-col :span="24">
      <el-input v-model="text" placeholder="请输入名称" clearable @keyup.enter.native="confirmInput">
        <template #append>
          <el-button @click="confirmInput"><i-ep-search /></el-button>
        </template>
      </el-input>
    </el-col>
  </el-row>
</template>

<script setup>
import { onMounted, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue'

const text = ref('');

function confirmInput(){
  console.log(1111);
}

onMounted(async () => {



})
onBeforeUnmount(() => {
})


</script>

<style></style>