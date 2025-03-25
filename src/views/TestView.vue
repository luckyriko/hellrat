<template>
  <div>
    <div data-tauri-drag-region style="height: 200px;">
      dsadsa
    </div>
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
  </div>
</template>

<script setup>
import Sortable from 'sortablejs'
import { onMounted } from 'vue'
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
onMounted(() => {
	setSort()
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
.sortable-ghost{
  color: white;
  background-color: red !important;
}
</style>