<template>
  <div>

    <!-- <div ref="drop" style="height: 200px;width: 100%;background-color: red;">
      拖动至此区域上传
    </div> -->
    <el-table data-tauri-drag-region :data="tableData" id="dragTable" border style="width: 800px;">
      <el-table-column prop="date" label="Date" width="180" />
      <el-table-column prop="name" label="Name" width="180" />
      <el-table-column prop="address" label="Address" />
      <el-table-column label="操作" width="100">
        <template #default>
          <div class="handle-drag">
            排序
          </div>
        </template>
      </el-table-column>
    </el-table>

    <div @click="createWin">创建窗口</div>


  </div>
</template>

<script setup>
import Sortable from 'sortablejs';
import { onMounted, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { ref } from 'vue'

import { WebviewWindow } from '@tauri-apps/api/webviewWindow'


async function createWin(params) {
  console.log('createWin');
  const webview = new WebviewWindow('upload', {
    url: '/upload',
    width: 400,
    height: 600,
    dragDropEnabled: true
    
  });
  webview.once('tauri://created', function () {
    // webview successfully created
  });
  webview.once('tauri://error', function (e) {
    // an error happened creating the webview
    console.log(e);
  });


}



let unlisten = null;
function setSort() {
  const el = document.querySelector('#dragTable table tbody')
  new Sortable(el, {
    sort: true,
    animation: 150,
    handle: '.handle-drag',
    ghostClass: 'sortable-ghost',
    easing: 'cubic-bezier(1, 0, 0, 1)',
    onStart: (item) => {
      // console.log(item);
    },
    onEnd: (e) => {
      const targetRow = tableData.splice(e.oldIndex, 1)[0]
      tableData.splice(e.newIndex, 0, targetRow)
      console.log(tableData)

      submitSortResult()

    },
  })
}

const submitSortResult = () => {
  console.log('submitSortResult')
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
  setSort();
  unlisten = await getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'over') {
      // console.log('User hovering', event.payload.position);
    } else if (event.payload.type === 'drop') {
      console.log('User dropped', event.payload.paths);
      let zipPath = event.payload.paths[0];
      console.log('--event.payload.paths---', event.payload.paths);
      // unzipFile(zipPath)
    } else {
      console.log('File drop cancelled');
    }
  });


})
onBeforeUnmount(() => {
  unlisten();
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