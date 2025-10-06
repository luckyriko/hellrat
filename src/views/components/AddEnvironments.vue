<template>
  <!-- el-dialog 组件，使用 v-model 绑定 props.visible 以实现双向数据绑定 -->
  <el-dialog v-model="showFlag" :title="$t('addEnvironments.title')" @close="handleClose" width="40%">
    <div class="container">
      <div class="add-box">
        <el-input v-model="input" :placeholder="$t('addEnvironments.placeholder')" clearable size="large" />
        <el-tooltip effect="dark" :content="$t('addEnvironments.add')" placement="top">
          <el-button :icon="CirclePlus" @click="addEnv" text style="font-size: 20px;" />
        </el-tooltip>
      </div>

      <div>
        <div v-for="item in list" :key="item.id" class="list">

          <div class="title">
            <div v-if="!item.editFlag">{{ item.name }}</div>
            <div v-else>
              <el-input v-model="item.editName" :placeholder="$t('addEnvironments.placeholder')">
                <template #append>
                  <el-button :icon="Select" type="primary" @click="editEnv(item)" />
                </template>
              </el-input>
            </div>

            <div v-if="item.activate == 1" style="margin-left: 10px;">
              <el-icon color="red" size="20">
                <Smoking />
              </el-icon>
            </div>
          </div>

          <div>
            <el-tooltip effect="dark"
              :content="item.editFlag ? $t('addEnvironments.cancelEdit') : $t('addEnvironments.edit')" placement="top">
              <el-button :icon="item.editFlag ? Close : EditPen" @click="item.editFlag = !item.editFlag" text
                style="font-size: 18px;" />
            </el-tooltip>

            <el-tooltip effect="dark" :content="$t('addEnvironments.copy')" placement="top">
              <el-button :icon="DocumentCopy" @click="copyEnv(item)" text style="font-size: 18px;" />
            </el-tooltip>

            <el-tooltip v-if="item.name != 'default'" effect="dark" :content="$t('addEnvironments.delete')"
              placement="top">
              <el-button :icon="Delete" @click="delEnv(item)" text style="font-size: 18px;" />
            </el-tooltip>

          </div>
        </div>


      </div>
    </div>
    <!-- 底部插槽，提供关闭按钮 -->
    <template #footer>
      <el-button @click="handleClose">{{ $t('addEnvironments.cancel') }}</el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, watch } from 'vue';
import { CirclePlus, Delete, EditPen, DocumentCopy, Select, Smoking, Close } from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const props = defineProps({
  list: {
    type: Array,
    required: true,
    default: []
  }
})
const emit = defineEmits(['refresh', 'close']);
// watch(
//   () => props.list,
//   (newVal) => {
//     console.log("检测到env_list变化:", newVal);
//   },
//   { deep: true }
// );

const input = ref('')

const showFlag = defineModel();

const addEnv = async () => {
  if (!input.value) {
    ElMessage.error(t('addEnvironments.addTip'));
    return;
  }
  try {
    await invoke("add_environment", { name: input.value });
    emit('refresh');
    input.value = '';
  } catch (error) {
    console.log(error);
    ElMessage.error(t('addEnvironments.addFail') + String(error))

  }
}

const editEnv = async (item) => {
  if (!item.editName) {
    ElMessage.error(t('addEnvironments.editTip'));
    return;
  }
  if (item.editName == item.name) {
    item.editFlag = false;
    return;
  }
  try {
    await invoke("update_environment_name", { id: item.id, name: item.editName });
    item.editFlag = false;
    emit('refresh');
  } catch (error) {
    console.log(error);
    ElMessage.error(t('addEnvironments.editFail') + String(error))

  }
}


const copyEnv = async (item) => {
  try {

    let name = item.name + '_copy';
    let envItem = props.list.find(x => x.name == name);
    if (envItem) {
      ElMessage.error($t('addEnvironments.sameNameError', { name }))
      return;
    }
    await invoke("copy_environment", { id: item.id, name: item.name });
    emit('refresh');
  } catch (error) {
    console.log(error);
    ElMessage.error(t('addEnvironments.copyError') + String(error))

  }
}

const delEnv = async (item) => {
  try {
    if (item.activate == 1) {
      ElMessage.error(t('addEnvironments.deleteTip'));
      return
    }
    await invoke("delete_environment", { id: item.id });
    emit('refresh');
  } catch (error) {
    console.log(error);
    ElMessage.error(t('addEnvironments.deleteError') + String(error));

  }
}

// 关闭对话框时触发的函数
const handleClose = () => {
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

.container {
  box-sizing: border-box;
  border: 1px solid #ececec;
  padding: 20px;
  border-radius: 6px;
}

.add-box {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 10px;
  /* border-bottom: 1px solid #ececec; */
}

.list {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  border-bottom: 1px solid #ececec;

  .title {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    font-size: 1rem;
    color: #333;
  }

  :deep(.el-button) {
    padding: 0 5px;
  }
}
</style>
