<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">安装</div>
        </el-col>
        <!-- <el-col :span="24">
          <div class="desc">1. 请先去设置里配置游戏data目录和mod存档目录；目前只支持单mod目录添加；</div>
        </el-col>
        <el-col :span="24">
          <div class="desc">2. 目前只支持以 9ba626afa44a3aa3
            开头的.patch_xxx、.patch_xxx.gpu_resources（可无）、.patch_xxx.stream（可无）文件的安装，其中xxx必须为数字，其它文件会忽略；</div>
        </el-col> -->
      </el-row>

      <el-tree style="max-width: 600px; color: red; margin-bottom: 20px;margin-left: 12px;" :data="data" :props="defaultProps"
        @node-click="handleNodeClick" />

      <el-row>
        <el-col :span="18">
          <el-form ref="formRef" :model="form" label-width="100px" style="max-width: 700px" :rules="rules">
            <el-form-item label="Mod目录" prop="directory">
              <el-col :span="17">
                <el-input v-model.trim="form.directory" />
              </el-col>
              <el-col :span="2" :offset="1">
                <el-button type="primary" @click="onSelect">选择</el-button>
              </el-col>
              <el-col :span="2" :offset="1">
                <el-button type="primary" @click="openDir(form.directory)">打开</el-button>
              </el-col>
            </el-form-item>

            <el-form-item label="名称" prop="name">
              <el-input v-model.trim="form.name" />
            </el-form-item>

            <el-form-item label="类型" prop="mod_type">
              <el-radio-group v-model="form.mod_type">
                <el-radio value="model">模型</el-radio>
                <el-radio value="voice">音频/杂项</el-radio>
              </el-radio-group>
            </el-form-item>

            <el-form-item label="安装" prop="activate">
              <el-switch v-model="form.activate" />
            </el-form-item>

            <el-form-item label="预览图" prop="preview">
              <div v-if="previewImg" class="image" @click="selectPreViewImg">
                <el-image style="height: 150px" :src="previewImg" fit="fill" />
                <el-icon class="close-icon" size="30" color="red"
                  @click.stop="removeImg"><i-ep-circle-close-filled /></el-icon>
              </div>
              <div v-else class="image" @click="selectPreViewImg">
                <el-icon size="20" color="#666"><i-ep-plus /></el-icon>
              </div>
              <el-input v-model.trim="form.preview" type="hidden" />

            </el-form-item>

            <el-form-item label="作者" prop="author">
              <el-input v-model.trim="form.author" />
            </el-form-item>

            <el-form-item label="链接" prop="link">
              <el-input v-model.trim="form.link" />
            </el-form-item>

            <el-form-item label="详情" prop="desc">
              <el-input v-model="form.desc" type="textarea" />
            </el-form-item>

            <el-form-item>
              <el-button type="primary" :loading="loadingFlag" @click="onSubmit(formRef)">{{ buttonText }}</el-button>
              <el-button @click="resetForm(formRef)">重置</el-button>
            </el-form-item>
          </el-form>
        </el-col>
        <el-col :span="6" style="padding-left: 10px;">
          <div class="detail" v-show="modFiles && modFiles.length > 0">目录预览</div>
          <p style="font-size: 10px;" v-for="(item, index) in modFiles" :key="index">{{ item }}</p>
        </el-col>
      </el-row>
    </div>
  </el-scrollbar>

</template>

<script setup>
import { platform } from '@tauri-apps/plugin-os';
import { ref, reactive, onMounted, toRaw, computed } from 'vue'
import { open } from '@tauri-apps/plugin-dialog';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { join, basename } from '@tauri-apps/api/path';

const loadingFlag = ref(false);
const modsDir = ref("");
const gameDataDir = ref("");

onMounted(async () => {
  const loading = ElLoading.service({
    lock: true,
    text: 'Loading',
    background: 'rgba(0, 0, 0, 0.7)',
  })

  try {
    const contents = await readTextFile('config.json', {
      baseDir: BaseDirectory.AppConfig,
    });
    if (contents) {
      const config = JSON.parse(contents);
      gameDataDir.value = config.gameDataDir || '';
      modsDir.value = config.modsDir || '';
      if (!gameDataDir.value || !modsDir.value) {
        ElMessage.error('请先去设置里添加游戏data目录和mod存档目录')
      }
    } else {
      ElMessage.error('请先去设置里添加游戏data目录和mod存档目录')
    }

  } catch (error) {
    console.error('获取配置文件失败:', error);
    ElMessage.error(error || '获取配置文件失败')

  } finally {
    loading.close()

  }

});

const handleNodeClick = (data) => {
  console.log(data)
}

const data = [
  {
    label: 'Mod目录示例，不支持嵌套目录，点击查看详情',
    children: [
      {
        label: '9ba626afa44a3aa3.patch_233',
      },
      {
        label: '9ba626afa44a3aa3.patch_666',
      },
      {
        label: '9ba626afa44a3aa3.patch_666.gpu_resources',
      },
      {
        label: '9ba626afa44a3aa3.patch_888',
      },
      {
        label: '9ba626afa44a3aa3.patch_888.gpu_resources',
      },
      {
        label: '9ba626afa44a3aa3.patch_888.stream',
      },
    ],
  }
]

const defaultProps = {
  children: 'children',
  label: 'label',
}


const formRef = ref();

const form = reactive({
  name: '',
  directory: '',
  author: '',
  activate: false,
  link: '',
  mod_type: 'model',
  desc: '',
  preview: ''
})

const buttonText = computed(() => form.activate ? '安装' : '存档');

const previewImg = computed(() => form.preview ? convertFileSrc(form.preview) : '');

const selectPreViewImg = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: false,
    title: "请选择预览图",
  });


  if (selectedPath) {
    form.preview = selectedPath;
  }

}

const removeImg = () => {
  form.preview = '';
}

const modFiles = ref([]);

const onSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: "请选择要安装的MOD目录",
  });


  if (selectedPath) {
    // const currentPlatform = platform();
    const lastSegment = await basename(selectedPath);
    form.directory = selectedPath;
    form.name = lastSegment;

    try {
      const modInfo = await invoke("get_mod_info", { folderPath: selectedPath });
      console.log('---modInfo---', modInfo);
      form.preview = modInfo.first_image_path;
      form.mod_type = modInfo.mod_type;
      modFiles.value = modInfo.files;

    } catch (error) {
      console.error("Error:", error);
    }
  }

}

async function openDir(path) {
  if (!path) {
    ElMessage.error('没有设置路径')
    return;
  }
  try {
    await invoke('open_folder', { path });
  } catch (error) {
    console.error('Failed to open folder:', error);
  }
}

const checkName = async (rule, value, callback) => {
  if (!value) {
    return callback(new Error('请输入mod名称'))
  }
  try {
    let flag = await invoke('check_mod_name', { name: value });
    if (flag) {
      callback()
    } else {
      callback(new Error('Mod名称重复，请重新命名！'))
    }
  } catch (error) {
    console.error('Failed to check name:', error);
    callback()

  }

}


const rules = reactive({
  name: [
    { required: true, validator: checkName, trigger: ['blur', 'change'] }
  ],
  directory: [
    { required: true, message: '请选择mod目录', trigger: ['blur', 'change'] },
  ],
  mod_type: [
    { required: true, message: '请选择mod类型', trigger: ['blur', 'change'] },
  ]
})

const onSubmit = async (formEl) => {
  if (!gameDataDir.value || !modsDir.value) {
    ElMessage.error('请先去设置里添加游戏data目录和mod存档目录');
    return;
  }

  if (!formEl) return;
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      loadingFlag.value = true;

      try {
        // 从下载目录copy到存档目录
        // 如果安装 从下载目录copy到游戏data目录并自动重命名排序
        let down_dir = form.directory;
        let record_dir = await join(modsDir.value, form.name);
        let data_dir = gameDataDir.value;
        let mod_info = toRaw(form);
        mod_info.memo = mod_info.name;
        // console.log('---从下载目录移动到游戏data目录并自动重命名排序---', {
        //   down_dir, record_dir, data_dir, mod_info
        // });

        await invoke('down_copy_and_rename_files', { down_dir, record_dir, data_dir, mod_info });
        console.log('down_copy_and_rename_files successfully!');

        ElMessage({
          message: buttonText.value + '成功',
          type: 'success',
        })
        // 重置表单
        resetForm(formEl);

      } catch (error) {
        console.error('Failed to submit:', error);
        ElMessage.error(String(error) || '提交失败')

      } finally {

        loadingFlag.value = false;
      }
    } else {
      console.log('error submit!', fields)
    }
  })
}

const resetForm = (formEl) => {
  if (!formEl) return
  formEl.resetFields();
}

</script>

<style scoped lang="scss">
.container {
  padding: 20px 10px;
}

.title {
  font-size: 20px;
  font-weight: bold;
  margin-bottom: 10px;
  margin-left: 10px;
}

.detail {
  font-size: 16px;
  font-weight: bold;
  margin-bottom: 10px;
  color: #333;
}

.desc {
  font-size: 15px;
  color: red;
  margin-bottom: 10px;

}

.image {
  min-width: 150px;
  height: 150px;
  line-height: 150px;
  text-align: center;
  border: dashed 1px #999;
  position: relative;

  .close-icon {
    position: absolute;
    right: -10px;
    top: -10px;
  }
}
</style>
