<template>
  <!-- el-dialog 组件，使用 v-model 绑定 props.visible 以实现双向数据绑定 -->
  <el-dialog v-model="showFlag" title="自定义对话框" @close="handleClose" width="90%" top="5vh">
    <p>请开始你的选择把。</p>

    <el-scrollbar max-height="66vh">
      <el-collapse v-model="activeNames" :before-collapse="beforeCollapse" :accordion="true" class="content">
        <el-collapse-item :name="index" v-for="(item, index) in options" :key="index">
          <template #title>
            <div class="collapse-box">
              <el-checkbox v-model="item.Activate" size="large">
                <template #default>
                  <div class="checkbox-box">

                    <el-popover :width="500" placement="right" v-if="item.Image"
                      popper-style="box-shadow: rgb(14 18 22 / 35%) 0px 10px 38px -10px, rgb(14 18 22 / 20%) 0px 10px 20px -15px; padding: 20px;">
                      <!-- 图片图标 -->
                      <template #reference>
                        <el-image :src="item.previewPath" fit="fill"
                          style="max-width: 260px; height: 100px; margin-right: 20px;" />
                      </template>
                      <!-- 图片内容 -->
                      <template #default>
                        <el-image style="width: 450px" :src="item.previewPath" fit="fill" />
                      </template>
                    </el-popover>

                    {{ `${index + 1}.` }} {{ item.Name }}
                  </div>
                </template>
              </el-checkbox>
              <div @click="handleClick(index)" style="flex: 1;" v-if="item.SubOptions && item.SubOptions.length > 0">
                <span v-if="item.Description">
                  （{{ item.Description }}）
                </span>
              </div>
              <div v-else>
                <span v-if="item.Description">
                  （{{ item.Description }}）
                </span>
              </div>

            </div>
          </template>

          <el-radio-group v-model="item.SubOptionValue" v-if="item.SubOptions && item.SubOptions.length > 0"
            class="radio">
            <el-radio :value="idx" v-for="(itm, idx) in item.SubOptions" :key="idx" border style="margin-bottom: 10px;">
              <template #default>
                <div class="radio-box">
                  <el-popover :width="480" placement="right" v-if="itm.Image"
                    popper-style="box-shadow: rgb(14 18 22 / 35%) 0px 10px 38px -10px, rgb(14 18 22 / 20%) 0px 10px 20px -15px; padding: 20px;">
                    <!-- 图片图标 -->
                    <template #reference>
                      <el-image :src="itm.previewPath" fit="fill"
                        style="max-width: 260px; height: 100px; margin-right: 20px;" />
                    </template>
                    <!-- 图片内容 -->
                    <template #default>
                      <el-image style="width: 440px" :src="itm.previewPath" fit="fill" />
                    </template>
                  </el-popover>

                  {{ itm.Name }}
                  <span v-if="itm.Description">
                    （{{ itm.Description }}）
                  </span>
                </div>
              </template>

            </el-radio>

          </el-radio-group>

          <template #icon>
            <span v-if="item.SubOptions && item.SubOptions.length > 0">
              <el-icon size="20" @click="handleClick(index)"><i-ep-edit /></el-icon>
            </span>
            <span v-else></span>
          </template>

        </el-collapse-item>

      </el-collapse>
    </el-scrollbar>

    <!-- 底部插槽，提供关闭按钮 -->
    <template #footer>
      <el-button @click="handleClose">取消</el-button>
      <el-button type="primary" :loading="loadingFlag" @click="onSubmit">
        确认
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, toRaw, reactive, onMounted, onUpdated, computed, watchEffect } from 'vue';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { join } from '@tauri-apps/api/path';

const props = defineProps(['id', 'modsStorePath', 'envId'])
// const props = defineProps({
//   visible: {
//     type: Boolean,
//     required: true,
//     default: false
//   },
// })
onMounted(() => {
  console.log(`the component is now mounted:`, props)
})


watchEffect(() => {
  // 在 3.5 之前只运行一次
  // 在 3.5+ 中prop变化时重新执行
  console.log(props.id)

})

onUpdated(() => {
  console.log('SelectOptions onUpdated', props);
  if (props.id && showFlag.value) {
    getModInfo(props.id)
  }
})

const loadingFlag = ref(false);
const options = ref([]);
const mod = ref({});

const getModInfo = async (id) => {
  try {
    options.value = [];
    mod.value = await invoke("get_env_mod_info", { id, env_id: props.envId });
    console.log('---mod----', mod.value);

    if (!mod.value?.env_mod_options) return;
    let options_temp = JSON.parse(mod.value.env_mod_options);

    await Promise.allSettled(options_temp.map(async element => {
      if (!element.hasOwnProperty('Activate')) {
        element.Activate = false;
      }
      if (!element.hasOwnProperty('previewPath')) {
        element.previewPath = element.Image ? await getPreviewPath(element.Image) : '';
      }
      if (element.SubOptions && element.SubOptions.length > 0) {
        if (!element.hasOwnProperty('SubOptionValue')) {
          element.SubOptionValue = 0;
        }
        for (const item of element.SubOptions) {
          if (!item.hasOwnProperty('previewPath')) {
            item.previewPath = item.Image ? await getPreviewPath(item.Image) : '';
          }
        }
      }

    }));

    options.value = options_temp;
    console.log(options.value);

  } catch (e) {
    console.log(e);
  }

}

const onSubmit = async () => {
  try {
    const optionsVal = toRaw(options.value)
    console.log(optionsVal);

    await invoke("update_environment_mod_options", { id: mod.value.env_mod_id, options: JSON.stringify(optionsVal) });
    ElMessage({
      message: '修改成功',
      type: 'success',
    })
    handleClose();

  } catch (error) {
    console.error("更新失败", error);
    ElMessage.error(error || 'Oops, this is a error message.')

  }
}

const getPreviewPath = async (imageName) => {
  if (props.modsStorePath && mod.value.path && imageName) {
    let imgPath = await join(props.modsStorePath, mod.value.path, '/' + imageName);
    const assetUrl = convertFileSrc(imgPath);
    // console.log(imgPath);
    return assetUrl;
  } else {
    return '';
  }
}

const checked1 = ref(true)

const showFlag = defineModel();
const before = ref(false);
const activeNames = ref(NaN);

const modsData = ref([111,]);
const beforeCollapse = () => {
  return false

}

const handleClick = (value) => {
  console.log('---handleClick---', value);
  activeNames.value = activeNames.value === value ? NaN : value;
}
const radio = ref(3)
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
.content {
  box-sizing: border-box;
  padding: 0 30px 0 10px;

}

.radio {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  margin-left: 20px;
}

:deep(.el-collapse-item__header) {
  height: auto;
  line-height: normal;
}

:deep(.el-collapse-item__title) {
  display: block;
  color: red;
}

:deep(.el-checkbox__label) {
  display: block;
}

:deep(.el-checkbox) {
  height: auto;
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

:deep(.el-radio) {
  height: auto;
}

.collapse-box {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 20px 0;
}

.checkbox-box {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.radio-box {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 20px 0;

}
</style>
