<template>
  <div class="menu" v-if="show" ref="menuRef">
    <div class="menu-item" v-for="(item, index) in mentList" :key="index" @click="handleClick(item)">
      {{ item.label }}
    </div>
  </div>
</template>
<script setup>
import { nextTick, ref } from 'vue'

defineProps({
  mentList: {
    type: Array,
    default: () => [],
  },
})

const emit = defineEmits(['selectLabel']);

const show = ref(false)

//点击菜单项
const handleClick = (item) => {
  // alert(name)
  show.value = false
  emit('selectLabel', item);
}


let menuRef = ref()
//获取并设置菜单的位置
const setPosition = (x, y) => {
  nextTick(() => {
    let dom = menuRef.value
    dom.style.left = x + 'px'
    dom.style.top = y + 'px'
    dom.style.height = 'fit-content'
  })
}
//暴露数据
defineExpose({
  show,
  setPosition,
})
</script>
<style lang="scss" scoped>
.menu {
  width: 100px;
  padding-top: 10px;
  padding-bottom: 10px;
  border-radius: 10px;
  background: #ffffff;
  height: 0px;
  position: absolute;
  z-index: 1000;

  .menu-item {
    font-size: 16px;
    width: 100%;
    text-align: center;
    padding-top: 4px;
    padding-bottom: 4px;
    cursor: pointer;
    transition: 0.5s;
  }

  .menu-item:hover {
    background: #e2e2e2;
    transition: 0.5s;
  }
}
</style>
