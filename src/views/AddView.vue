<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">安装Mod</div>
        </el-col>
        <el-col :span="24">
          <div class="desc">1. 请先去设置里配置游戏data目录和mod存档目录；目前只支持单mod目录添加；</div>
        </el-col>
        <el-col :span="24">
          <div class="desc">2. 目前只支持以 9ba626afa44a3aa3
            开头的.patch_xxx、.patch_xxx.gpu_resources（可无）、.patch_xxx.stream（可无）文件的安装，其中xxx必须为数字，其它文件会忽略；</div>
        </el-col>
        <el-col :span="24">
          <div class="desc">3. 无需删除游戏data目录里之前添加过的mod；删除mod需手动去游戏data目录中删除；</div>
        </el-col>
        <el-col :span="24">
          <div class="desc">4. 目前没有记录mod是否已经被安装过了，请勿重复安装同一个mod；</div>
        </el-col>
        <el-col :span="24">
          <div class="desc">5. 示例：使用时请选择"亚里亚玫红痛车"这个mod目录，不支持嵌套目录读取，文件结构如下：</div>
        </el-col>
      </el-row>

      <el-tree style="max-width: 600px; color: red; margin-bottom: 20px;" :data="data" :props="defaultProps"
        @node-click="handleNodeClick" />

      <el-form ref="formRef" :model="form" label-width="100px" style="max-width: 700px" :rules="rules">
        <el-form-item label="Mod文件夹" prop="directory">
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

        <el-form-item label="类型">
          <el-radio-group v-model="form.type">
            <el-radio value="model">模型</el-radio>
            <!-- <el-radio value="voice">音效</el-radio> -->
          </el-radio-group>
        </el-form-item>

        <!-- <el-form-item label="启用">
          <el-switch v-model="form.activate" />
        </el-form-item>

        <el-form-item label="作者" prop="author">
          <el-input v-model.trim="form.author" />
        </el-form-item>

        <el-form-item label="链接" prop="link">
          <el-input v-model.trim="form.link" />
        </el-form-item>

        <el-form-item label="详情" prop="desc">
          <el-input v-model="form.desc" type="textarea" />
        </el-form-item> -->
        <el-form-item>
          <el-button type="primary" :loading="loadingFlag" @click="onSubmit(formRef)">安装</el-button>
          <el-button @click="resetForm(formRef)">重置</el-button>
        </el-form-item>
      </el-form>
    </div>
  </el-scrollbar>

</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

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
    label: '亚里亚玫红痛车(Mod目录)',
    children: [
      {
        label: '9ba626afa44a3aa3.patch_0',
      },
      {
        label: '9ba626afa44a3aa3.patch_666',
      },
      {
        label: '9ba626afa44a3aa3.patch_666.gpu_resources',
      },
      {
        label: '9ba626afa44a3aa3.patch_666.stream',
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
  activate: true,
  link: '',
  type: 'model',
  desc: '',
})

const onSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: "请选择要安装的MOD文件夹",
  });


  if (selectedPath) {
    const lastPart = selectedPath.split("\\").pop();
    form.directory = selectedPath;
    form.name = lastPart;

  }

}

async function openDir(path) {
  if (!path) {
    ElMessage.error('没有设置路径')
    return;
  }
  try {
    await invoke('open_win_folder', { path });
  } catch (error) {
    console.error('Failed to open folder:', error);
  }
}

const rules = reactive({
  name: [
    { required: true, message: '请输入mod名称', trigger: ['blur', 'change'] },
    { min: 1, max: 255, message: '最少1个字符，最长255个字符', trigger: ['blur', 'change'] },
  ],
  directory: [
    { required: true, message: '请选择mod文件夹', trigger: ['blur', 'change'] },
  ]
})

const onSubmit = async (formEl) => {
  if (!formEl) return;
  await formEl.validate(async (valid, fields) => {
    if (valid) {

      if (!gameDataDir.value || !modsDir.value) {
        ElMessage.error('请先去设置里添加游戏data目录和mod存档目录');
        return;
      }
      loadingFlag.value = true;

      console.log(666666666);
      try {
        // 从下载目录移动到存档目录
        let source_dir = form.directory;
        let target_dir = modsDir.value + '\\' + form.name;
        console.log('---从下载目录移动到存档目录---', source_dir, target_dir);
        await invoke('copy_files', { source_dir, target_dir });
        console.log('copy_files successfully!');

        // 从下载目录移动到游戏data目录并自动重命名排序
        let src_dir = form.directory;
        let data_dir = gameDataDir.value;
        let prefix = '9ba626afa44a3aa3'
        console.log('---从下载目录移动到游戏data目录并自动重命名排序---', src_dir, data_dir);
        await invoke('transfer_and_rename_files', { src_dir, data_dir, prefix });
        console.log('transfer_and_rename_files successfully!');

        ElMessage({
          message: '安装成功',
          type: 'success',
        })
        // 重置表单
        resetForm(formEl);

      } catch (error) {
        console.error('Failed to submit:', error);
        ElMessage.error(error || '添加失败')

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
  formEl.resetFields()
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

.desc {
  font-size: 15px;
  color: red;
  margin-bottom: 10px;

}
</style>
