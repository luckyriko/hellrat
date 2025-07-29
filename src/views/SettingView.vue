<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">游戏设置</div>
        </el-col>
      </el-row>

      <el-form :model="form" label-width="150px" style="max-width: 760px" label-position="left" ref="modFormRef"
        :rules="modFormRules">
        <el-form-item label="设置存储路径" prop="configDir">
          <el-col :span="18">
            <el-input v-model.trim="form.configDir" disabled />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.configDir)">打开</el-button>
          </el-col>
        </el-form-item>

        <el-form-item label="游戏data路径" prop="gameDataDir">
          <el-col :span="18">
            <el-input v-model.trim="form.gameDataDir" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onSelect">选择</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.gameDataDir)">打开</el-button>
          </el-col>
        </el-form-item>

        <el-form-item label="Mods存档路径" prop="modsDir">
          <el-col :span="18">
            <el-input v-model.trim="form.modsDir" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onModsSelect">选择</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.modsDir)">打开</el-button>
          </el-col>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="onSubmit(modFormRef)">保存</el-button>
          <!-- <el-button @click="resetSubmit(modFormRef)">重置</el-button> -->

        </el-form-item>
      </el-form>


      <el-row>
        <el-col :span="24">
          <div class="title">输入设置</div>
        </el-col>
      </el-row>

      <el-form :model="keyboard" label-width="150px" style="max-width: 760px" label-position="left"
        ref="keyboardFormRef" :rules="keyboardFormRules">
        <el-form-item label="唤起/隐藏按键" prop="shortcut">
          <el-col :span="18">
            <el-input v-model.trim="keyboard.shortcut" disabled />
          </el-col>
          <!-- <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.configDir)">打开</el-button>
          </el-col> -->
        </el-form-item>

        <el-form-item label="窗口宽高w-h" required>
          <el-col :span="8">
            <el-form-item prop="width">
              <el-input v-model="keyboard.width" clearable inputmode="decimal" />
            </el-form-item>
          </el-col>
          <el-col class="text-center" :span="2">
            <span class="text-gray-500">-</span>
          </el-col>
          <el-col :span="8">
            <el-form-item prop="height">
              <el-input v-model.trim="keyboard.height" clearable inputmode="decimal" />
            </el-form-item>
          </el-col>
        </el-form-item>

        <el-form-item label="窗口坐标x-y" required>
          <el-col :span="8">
            <el-form-item prop="x">
              <el-input v-model.trim="keyboard.x" clearable inputmode="decimal" />
            </el-form-item>
          </el-col>
          <el-col class="text-center" :span="2">
            <span class="text-gray-500">-</span>
          </el-col>
          <el-col :span="8">
            <el-form-item prop="y">
              <el-input v-model.trim="keyboard.y" clearable inputmode="decimal" />
            </el-form-item>
          </el-col>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="onSubmit(keyboardFormRef)">保存</el-button>
          <el-button @click="resetSubmit(keyboardFormRef)">重置</el-button>

        </el-form-item>
      </el-form>

    </div>
  </el-scrollbar>

</template>

<script setup>
import { ref, reactive, onMounted } from 'vue';
import { platform } from '@tauri-apps/plugin-os';
import { open, save } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { appConfigDir, appDataDir, homeDir, join } from '@tauri-apps/api/path';
import { exists, mkdir, create, readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

const modFormRef = ref();
const keyboardFormRef = ref();

const form = reactive({
  configDir: '',
  gameDataDir: '',
  modsDir: ''
})

const keyboard = reactive({
  shortcut: 'ctrl + space',
  width: '340.0',
  height: '40.0',
  x: '1335.0',
  y: '960.0',
})


const modFormRules = reactive({
  configDir: [
    { required: true, message: '请选择配置目录', trigger: 'blur' }
  ],
  gameDataDir: [
    { required: true, message: '请选择游戏Data目录', trigger: 'blur' }
  ],
  modsDir: [
    { required: true, message: '请选择Mod存档目录', trigger: 'blur' }
  ],
})

const keyboardFormRules = reactive({
  shortcut: [
    { required: true, message: '请绑定快捷键', trigger: 'blur' }
  ],
  width: [
    { required: true, message: '请输入窗口宽度', trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: '必须是带一位小数的数字（如 1.0）',
      trigger: 'blur',
    }
  ],
  height: [
    { required: true, message: '请输入窗口高度', trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: '必须是带一位小数的数字（如 1.0）',
      trigger: 'blur',
    }
  ],
  x: [
    { required: true, message: '请输入窗口坐标x轴', trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: '必须是带一位小数的数字（如 1.0）',
      trigger: 'blur',
    }
  ],
  y: [
    { required: true, message: '请输入窗口坐标y轴', trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: '必须是带一位小数的数字（如 1.0）',
      trigger: 'blur',
    }
  ],
})

onMounted(async () => {
  const loading = ElLoading.service({
    lock: true,
    text: 'Loading',
    background: 'rgba(0, 0, 0, 0.7)',
  })
  try {
    const currentPlatform = platform();
    console.log('当前环境是 ', currentPlatform);

    // 获取APP配置目录的完整路径
    const appConfigDirPath = await appConfigDir();
    // const appDataDirPath = await appDataDir();
    // const homeDirPath = await homeDir();
    form.configDir = appConfigDirPath;
    // console.log('appConfigDir, appDataDir, homeDir Directory:', appConfigDirPath, appDataDirPath, homeDirPath);

    // 不存在配置目录则去创建
    const appConfigDirExists = await exists('', {
      baseDir: BaseDirectory.AppConfig,
    });
    // console.log('---appConfigDirExists---', appConfigDirExists);
    if (!appConfigDirExists) {
      await mkdir('', { baseDir: BaseDirectory.AppConfig })
    }

    // 不存在配置文件则去创建
    const flag = await exists('config.json', { baseDir: BaseDirectory.AppConfig });
    if (!flag) {
      // 使用join拼接目录可以抹平平台差异
      let modsDir = await join(appConfigDirPath, 'mods');

      const modsDirExists = await exists('mods', {
        baseDir: BaseDirectory.AppConfig,
      });
      if (!modsDirExists) {
        await mkdir('mods', { baseDir: BaseDirectory.AppConfig })
      }

      const file = await create('config.json', { baseDir: BaseDirectory.AppData });
      await file.write(new TextEncoder().encode(JSON.stringify({ gameDataDir: '', modsDir })));
      await file.close();

      form.modsDir = modsDir;
      return;
    }
    const contents = await readTextFile('config.json', {
      baseDir: BaseDirectory.AppConfig,
    });

    if (contents) {
      const config = JSON.parse(contents);
      form.gameDataDir = config.gameDataDir || '';
      form.modsDir = config.modsDir || '';

      keyboard.shortcut = config.keyboard_shortcut || keyboard.shortcut;
      keyboard.width = config.keyboard_width || keyboard.width;
      keyboard.height = config.keyboard_height || keyboard.height;
      keyboard.x = config.keyboard_x || keyboard.x;
      keyboard.y = config.keyboard_y || keyboard.y;
    }

  } catch (error) {
    console.error('配置文件初始化失败:', error);
    ElMessage.error(error || '配置文件初始化失败')

  } finally {
    loading.close();

  }

});

async function openDir(path) {
  if (!path) {
    ElMessage.error('没有设置路径')
    return;
  }
  try {
    await invoke('open_folder', { path });
    // console.log('Folder opened successfully!');
  } catch (error) {
    console.error('Failed to open folder:', error);
    ElMessage.error(error || 'Oops, this is a error message.')

  }
}

const onSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: "请选择游戏里的Mods安装目录",
  });
  if (selectedPath) {
    form.gameDataDir = selectedPath;
  }

}

const onModsSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: "请选择Mods存档目录",
  });
  if (selectedPath) {
    form.modsDir = selectedPath;
  }

}

const onSubmit = async (formEl) => {
  if (!formEl) return
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      const { gameDataDir, modsDir } = form;
      const { width, height, x, y } = keyboard;
      try {
        const contents = JSON.stringify({
          gameDataDir,
          modsDir,
          keyboard_width: width,
          keyboard_height: height,
          keyboard_x: x,
          keyboard_y: y
        });
        await writeTextFile('config.json', contents, {
          baseDir: BaseDirectory.AppConfig,
        });

        ElMessage({
          message: '保存成功',
          type: 'success',
        })

        await invoke('close_webview_window', { label: 'keyboard' });


      } catch (error) {
        console.error('配置文件写入失败:', error);
        ElMessage.error(error || '保存失败')

      }
    } else {
      console.log('error submit!', fields)
    }
  })

}

async function resetSubmit(formEl) {
  if (!formEl) return
  formEl.resetFields()
}
</script>

<style scoped lang="scss">
.container {
  padding: 20px;
}

.title {
  font-size: 20px;
  font-weight: bold;
  margin-bottom: 10px;
}

.text-center {
  text-align: center;
}
</style>
