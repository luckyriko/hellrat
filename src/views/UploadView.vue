<template>
  <div>
    <!-- <div data-tauri-drag-region style="height: 200px;">
      dsadsa
    </div> -->
    <div ref="drop" style="height: 200px;width: 100%;background-color: red;">
      拖动至此区域上传
    </div>


    <!-- <div class="drop-zone" @dragover.prevent @drop="handleDrop">
      <p>将文件拖到这里上传</p>
      <ul>
        <li v-for="(f, index) in filePaths" :key="index">
          {{ f }}
        </li>
      </ul>
    </div> -->
  </div>
</template>

<script setup>
import { onMounted, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue'








const filePaths = ref([])

function handleDrop(event) {
  event.preventDefault()

  const files = event.dataTransfer?.files
  if (!files || files.length === 0) return

  for (const file of files) {
    // Tauri 中的 file.path 是可用的！
    filePaths.value.push(file.path ?? file.name)
    console.log(file);

  }


  // ✅ 可以在这里调用后端 API 保存文件、移动文件等操作
}


async function unzipFile(zipPath) {
  if (!zipPath) {
    ElMessage.error('没有获取到文件路径')
    return;
  }
  try {
    await invoke('unzip_one_file', { zipPath });
    // console.log('Folder opened successfully!');
  } catch (error) {
    console.error('Failed to unzip folder:', error);
    ElMessage.error(error || 'Oops, this is a error message.')

  }
}

onMounted(async () => {



})
onBeforeUnmount(() => {
})

const tableData = [
  {
    date: '2016-05-03',
    name: 'Tom',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-02',
    name: 'Cilly',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-04',
    name: 'Linda',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-01',
    name: 'John',
    address: 'No. 189, Grove St, Los Angeles',
  },
]
</script>

<style>
.sortable-ghost {
  color: white;
  background-color: red !important;
}

.drop-zone {
  border: 2px dashed #888;
  padding: 40px;
  text-align: center;
  color: #555;
  min-height: 200px;
}
</style>