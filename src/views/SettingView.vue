<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">设置</div>
        </el-col>
      </el-row>

      <el-form :model="form" label-width="150px" style="max-width: 760px" label-position="left">
        <el-form-item label="配置存储路径">
          <el-col :span="18">
            <el-input v-model.trim="form.configDir" disabled />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.configDir)">打开</el-button>
          </el-col>
        </el-form-item>

        <el-form-item label="游戏data路径">
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

        <el-form-item label="Mods存档路径">
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
          <el-button type="primary" @click="onSubmit">保存</el-button>
        </el-form-item>
      </el-form>
    </div>
  </el-scrollbar>

</template>

<script setup>
import { ref, reactive, onMounted } from 'vue';
import { platform } from '@tauri-apps/plugin-os';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { appConfigDir, appDataDir, homeDir } from '@tauri-apps/api/path';
import { exists, mkdir, create, readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

const form = reactive({
  gameDataDir: '',
  configDir: '',
  modsDir: ''
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
      let modsDir = '';
      if (currentPlatform === 'windows') {
        modsDir = appConfigDirPath + '\\mods';
      } else{
        modsDir = appConfigDirPath + '/mods';
      }

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
    title: "请选择游戏里的Mods安装路径",
  });
  if (selectedPath) {
    form.gameDataDir = selectedPath;
  }

}

const onModsSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: "请选择Mods存档路径",
  });
  if (selectedPath) {
    form.modsDir = selectedPath;
  }

}

const onSubmit = async () => {
  const { gameDataDir, modsDir } = form;
  try {
    const contents = JSON.stringify({
      gameDataDir,
      modsDir
    });
    await writeTextFile('config.json', contents, {
      baseDir: BaseDirectory.AppConfig,
    });
    ElMessage({
      message: '保存成功',
      type: 'success',
    })

  } catch (error) {
    console.error('Failed to write the config file:', error);
    ElMessage.error(error || 'Oops, this is a error message.')

  }

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
