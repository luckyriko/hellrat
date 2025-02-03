<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">导入现有Mod到此软件</div>
        </el-col>
      </el-row>

      <el-form :model="form" label-width="auto" style="max-width: 600px">
        <el-form-item label="名称">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item label="类型">
          <el-radio-group v-model="form.resource">
            <el-radio value="default">默认</el-radio>
            <el-radio value="shanghai">人物</el-radio>
            <el-radio value="beijing">音效</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="启用">
          <el-switch v-model="form.delivery" />
        </el-form-item>
        <el-form-item label="详情">
          <el-input v-model="form.desc" type="textarea" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="onSubmit">创建</el-button>
          <el-button>重置</el-button>
        </el-form-item>
      </el-form>
    </div>
  </el-scrollbar>

</template>

<script lang="ts" setup>
import { reactive } from 'vue'

const form = reactive({
  name: '',
  region: '',
  date1: '',
  date2: '',
  delivery: false,
  type: [],
  resource: '',
  desc: '',
})

const onSubmit = () => {
  console.log('submit!')
}
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
