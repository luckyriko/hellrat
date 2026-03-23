<template>
  <!-- el-dialog 组件，使用 v-model 绑定 props.visible 以实现双向数据绑定 -->
  <el-dialog v-model="showFlag" :title="$t('selectMods.title')" @close="handleClose" width="60%" top="5vh">
    <el-button type="primary" @click="selectAll">{{ $t('selectMods.selectAll') }}</el-button>
    <el-button type="primary" @click="unselectAll">{{ $t('selectMods.deselectAll') }}</el-button>

    <el-scrollbar max-height="66vh" style="margin-top: 10px;">
      <el-checkbox-group v-model="checkList" @change="selectModsChange">
        <div v-for="item in list" :key="item.id">
          <div class="list">
            <el-checkbox :value="item.id" size="large" :disabled="item.env_add_flag == 1">
              <template #default>
                <div class="checkbox-box">
                  <el-image :src="item.previewPath" fit="fill"
                    style="max-width: 260px; height: 100px; margin-right: 20px;">
                    <template #error>
                      <div class="image-slot">
                        <el-icon><i-ep-picture /></el-icon>
                      </div>
                    </template>
                  </el-image>
                  <div>{{ item.name }}</div>
                </div>
              </template>
            </el-checkbox>
          </div>
        </div>
      </el-checkbox-group>

    </el-scrollbar>

    <!-- 底部插槽，提供关闭按钮 -->
    <template #footer>
      <el-button @click="handleClose">{{ $t('selectMods.cancel') }}</el-button>
      <el-button type="primary" @click="onSubmit">
        {{ $t('selectMods.confirm') }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from "vue-i18n";
const { t } = useI18n();

const props = defineProps(['list', 'envId'])
const showFlag = defineModel();
const checkList = ref([]);

const selectModsChange = async () => {
  // console.log(checkList.value);
}


const selectAll = async () => {
  // console.log(checkList.value);
  checkList.value = [];
  props.list.forEach(element => {
    if (element.env_add_flag != 1) {
      checkList.value.push(element.id);
    }
  });
}

const unselectAll = async () => {
  checkList.value = [];
}

const onSubmit = async () => {
  if (checkList.value.length == 0) {
    ElMessage.error(t('selectMods.tip'));
    return
  }

  try {
    await invoke("add_mods_to_environment", { env_id: props.envId, ids: checkList.value });
    ElMessage({
      message: t('selectMods.success'),
      type: 'success',
    })
    emit('refresh');
    handleClose();
  } catch (error) {
    console.error("添加失败", error);
    ElMessage.error(error || t('selectMods.fail'))

  }
}

// 定义事件，用于向父组件传递更新和关闭操作
const emit = defineEmits(['refresh', 'close']);

// 关闭对话框时触发的函数
const handleClose = () => {
  // 通过 update:visible 事件通知父组件更新 dialogVisible 状态
  // emit('update:visible', false);
  // 触发自定义 close 事件，父组件可以监听此事件执行额外逻辑
  emit('close');
};
</script>

<style scoped>
.list {
  box-sizing: border-box;
  padding-bottom: 10px;
  margin-bottom: 10px;
  margin-left: 20px;
  border-bottom: 1px solid #ececec;
}

.checkbox-box {
  display: flex;
  flex-direction: row;
  align-items: center;
  flex: 1;
}

.image-slot {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100px;
  background: var(--el-fill-color-light);
  color: var(--el-text-color-secondary);
  font-size: 30px;
  width: 100px;
}

:deep(.el-checkbox__label) {
  display: block;
}

:deep(.el-checkbox) {
  height: auto;
  width: 100%;
}

:deep(.el-checkbox.el-checkbox--large) {
  height: auto;
}

:deep(.el-radio__label) {
  display: block;
}

:deep(.el-checkbox.el-checkbox--large) {
  height: auto;
}
</style>
