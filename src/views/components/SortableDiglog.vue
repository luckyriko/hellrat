<template>
   <!-- el-dialog 组件，使用 v-model 绑定 props.visible 以实现双向数据绑定 -->
   <el-dialog v-model="showFlag" title="自定义对话框" @close="handleClose" width="30%">
    <p>这是一个使用组件形式的 Dialog。</p>

    <!-- 底部插槽，提供关闭按钮 -->
    <template #footer>
      <el-button @click="handleClose">关闭</el-button>
    </template>
  </el-dialog>
</template>

<script setup>
// import { ref, reactive, onMounted, toRaw, computed } from 'vue'

// const props = defineProps({
//   visible: {
//     type: Boolean,
//     required: true,
//     default: false
//   },
// })

const showFlag = defineModel();



// 定义事件，用于向父组件传递更新和关闭操作
const emit = defineEmits(['close']);

// 关闭对话框时触发的函数
const handleClose = () => {
  // 通过 update:visible 事件通知父组件更新 dialogVisible 状态
  // emit('update:visible', false);
  // 触发自定义 close 事件，父组件可以监听此事件执行额外逻辑
  emit('close');
};
</script>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {

  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
