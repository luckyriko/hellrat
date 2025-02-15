<template>
  <div class="container">
    <el-row>
      <el-col :span="24">
        <div class="title">Mod列表</div>
      </el-col>
    </el-row>
    <!-- 
    <h2>功能开发中...</h2>
    <h3>先去设置页配置目录。</h3>
    <h3>再去安装页安装mod，与手动安装效果一致。</h3>
    <h3>请使用Win10、Win11，不保证也没有测试过Win7、Win8可用性。</h3> -->

    <el-table ref="tableRef" :default-sort="{ prop: 'id', order: 'descending' }" :data="modsData" max-height="500"
      style="width: 100%" row-key="id" stripe lazy :load="load"
      :tree-props="{ children: 'children', hasChildren: 'hasChildren' }">
      <el-table-column prop="id" label="ID" sortable min-width="60"/>
      <el-table-column prop="name" label="名称" min-width="120"/>
      <el-table-column prop="mod_type" label="类型" :formatter="type"/>
      <el-table-column prop="activate" label="状态" :filters="[
        { text: '已安装', value: true },
        { text: '未安装', value: false },
      ]" :filter-method="filterHandler"
      filter-placement="bottom-end"

      >
        <template #default="scope">
          <el-tag :type="scope.row.activate ? 'success' : 'danger'" disable-transitions>{{ scope.row.activate ?
            '已安装' : '未安装'}}</el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="author" label="作者" />
      <!-- <el-table-column prop="link" label="链接" /> -->
      <el-table-column property="desc" label="详情" width="240" show-overflow-tooltip />
      <!-- <el-table-column prop="preview" label="预览图" /> -->
      <el-table-column fixed="right" label="Operations" min-width="120">
        <template #default="scope">
          <el-button link type="primary" size="small" @click.prevent="deleteRow(scope.$index)">
            Remove
          </el-button>
        </template>
      </el-table-column>
    </el-table>
    <el-button class="mt-4" style="width: 100%" @click="onAddItem">
      Add Item
    </el-button>
  </div>
</template>
<script setup>
import { ref, reactive, onMounted, toRaw, computed } from 'vue'
import { open } from '@tauri-apps/plugin-dialog';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';

let modsData = ref([]);

const tableRef = ref();

const clearFilter = () => {
  tableRef.value.clearFilter()
}

const filterHandler = (value, row) => {
  console.log(value, row.activate);
  return row.activate === value
}

const type = (row, column) => {
  return row.mod_type === 'model' ? '模型' : '音频';
}


async function getModsList() {
  const loading = ElLoading.service({
    lock: true,
    text: 'Loading',
    background: 'rgba(0, 0, 0, 0.7)',
  })

  try {
    modsData.value = await invoke("get_mod_records");
    console.log("get mod records:", modsData.value);
  } catch (error) {
    console.error("Error get records:", error);
    ElMessage.error('获取失败')

  } finally {
    loading.close();

  }
}

const deleteRow = (index) => {
  modsData.value.splice(index, 1)
}

const onAddItem = () => {
  modsData.value.push({
    id: 22,
    name: 'Tom',
    mod_type: 'California',
    activate: 'Los Angeles',
    desc: 'No. 189, Grove St, Los Angeles',
    author: 'CA 90036',
  })
}


onMounted(async () => {
  await getModsList();
})

const load = (
  row,
  treeNode,
  resolve
) => {
  setTimeout(() => {
    resolve([
      {
        id: 31,
        date: '2016-05-01',
        name: 'wangxiaohu',
        address: 'No. 189, Grove St, Los Angeles',
      },
      {
        id: 32,
        date: '2016-05-01',
        name: 'wangxiaohu',
        address: 'No. 189, Grove St, Los Angeles',
      },
    ])
  }, 1000)
}

const tableData1 = [
  {
    id: 1,
    date: '2016-05-02',
    name: 'wangxiaohu',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    id: 2,
    date: '2016-05-04',
    name: 'wangxiaohu',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    id: 3,
    date: '2016-05-01',
    name: 'wangxiaohu',
    hasChildren: true,
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    id: 4,
    date: '2016-05-03',
    name: 'wangxiaohu',
    address: 'No. 189, Grove St, Los Angeles',
  },
]
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
</style>