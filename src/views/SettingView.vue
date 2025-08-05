<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">游戏设置</div>
        </el-col>
      </el-row>

      <el-form :model="gameMod" label-width="150px" style="max-width: 760px" label-position="right" ref="gameModFormRef"
        :rules="gameModFormRules">
        <el-form-item label="软件配置目录" prop="app_config_path">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.app_config_path" disabled />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.app_config_path)">打开</el-button>
          </el-col>
        </el-form-item>

        <el-form-item label="游戏data目录" prop="game_data_path">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.game_data_path" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onSelect">选择</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.game_data_path)">打开</el-button>
          </el-col>
        </el-form-item>

        <el-form-item label="Mod存档目录" prop="mods_store_path">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.mods_store_path" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onModsSelect">选择</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.mods_store_path)">打开</el-button>
          </el-col>
        </el-form-item>

        <el-form-item label="临时缓存目录" prop="mods_temp_cache_path">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.mods_temp_cache_path" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onModsTempSelect">选择</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.mods_temp_cache_path)">打开</el-button>
          </el-col>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="onSubmit(gameModFormRef, 'game_mod')">保存</el-button>
          <!-- <el-button @click="resetSubmit(gameModFormRef)">重置</el-button> -->

        </el-form-item>
      </el-form>


      <el-row>
        <el-col :span="24">
          <div class="title">弹窗输入</div>
        </el-col>
      </el-row>

      <el-form :model="keyboard" label-width="150px" style="max-width: 760px" label-position="right"
        ref="keyboardFormRef" :rules="keyboardFormRules">
        <el-form-item label="开启/关闭功能" required prop="flag">
          <el-col :span="2">
            <el-switch v-model="keyboard.flag" inline-prompt
              style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949" active-text="Y" inactive-text="N" />
          </el-col>
          <el-col :span="10">
            <text style="color: red; font-size: 12px;">注意：软件重启后生效</text>
          </el-col>
        </el-form-item>
        <el-form-item label="唤起/隐藏按键" prop="shortcut">
          <el-col :span="18">
            <el-input v-model.trim="keyboard.shortcut" disabled />
          </el-col>
          <!-- <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.configDir)">打开</el-button>
          </el-col> -->
        </el-form-item>
        <el-form-item label="文字输入间隔" prop="typing_interval">
          <el-col :span="18">
            <el-input v-model.trim="keyboard.typing_interval" />
          </el-col>
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
          <el-button type="primary" @click="onSubmit(keyboardFormRef, 'keyboard')">保存</el-button>
          <el-button @click="resetSubmit(keyboardFormRef)">重置</el-button>

        </el-form-item>
      </el-form>

      <el-row>
        <el-col :span="24">
          <div class="title">便捷短语</div>
        </el-col>
      </el-row>

      <el-form :model="quicklyChat" label-width="150px" style="max-width: 760px" label-position="right"
        ref="quicklyChatFormRef" :rules="quicklyChatFormRules">
        <el-form-item label="开启/关闭功能" required prop="flag">
          <el-col :span="2">
            <el-switch v-model="quicklyChat.flag" inline-prompt
              style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949" active-text="Y" inactive-text="N" />
          </el-col>
          <el-col :span="10">
            <text style="color: red; font-size: 12px;">注意：软件重启后生效</text>
          </el-col>
        </el-form-item>
        <el-form-item label="唤起/隐藏按键" prop="shortcut">
          <el-col :span="18">
            <el-input v-model.trim="quicklyChat.shortcut" disabled />
          </el-col>
          <!-- <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.configDir)">打开</el-button>
          </el-col> -->
        </el-form-item>
        <el-form-item label="文字输入间隔" prop="typing_interval">
          <el-col :span="18">
            <el-input v-model.trim="quicklyChat.typing_interval" />
          </el-col>
        </el-form-item>
        <el-form-item label="窗口宽高w-h" required>
          <el-col :span="8">
            <el-form-item prop="width">
              <el-input v-model="quicklyChat.width" clearable inputmode="decimal" />
            </el-form-item>
          </el-col>
          <el-col class="text-center" :span="2">
            <span class="text-gray-500">-</span>
          </el-col>
          <el-col :span="8">
            <el-form-item prop="height">
              <el-input v-model.trim="quicklyChat.height" clearable inputmode="decimal" />
            </el-form-item>
          </el-col>
        </el-form-item>

        <el-form-item :label="`短语 ${index + 1}：`" v-for="(key, index) in chatKeys" :key="index" :prop="`chat.${key}`">
          <el-col :span="18">
            <el-input v-model.trim="quicklyChat['chat'][key]" clearable />
          </el-col>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="onSubmit(quicklyChatFormRef, 'quickly_chat')">保存</el-button>
          <el-button @click="resetSubmit(quicklyChatFormRef)">重置</el-button>
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
import { Store } from '@tauri-apps/plugin-store'

const gameModFormRef = ref();
const keyboardFormRef = ref();
const quicklyChatFormRef = ref();

const gameMod = reactive({
  app_config_path: '',
  game_data_path: '',
  mods_store_path: '',
  mods_temp_cache_path: ''
})

const keyboard = reactive({
  typing_interval: '10',
  flag: true,
  shortcut: 'ctrl + space',
  width: '340.0',
  height: '40.0',
  x: '1335.0',
  y: '960.0',
})

const quicklyChat = reactive({
  typing_interval: '10',
  flag: true,
  shortcut: "ctrl + p",
  width: "800.0",
  height: "600.0",
  chat: {
    chat1: '你好！',
    chat2: '再见！',
    chat3: 'Ciallo～(∠・ω< )⌒☆',
    chat4: '',
    chat5: '',
    chat6: '',
    chat7: '',
    chat8: '',
    chat9: '',
  }
})
const chatKeys = ['chat1', 'chat2', 'chat3', 'chat4', 'chat5', 'chat6', 'chat7', 'chat8', 'chat9']


const gameModFormRules = reactive({
  app_config_path: [
    { required: true, message: '请选择软件配置目录', trigger: 'blur' }
  ],
  game_data_path: [
    { required: true, message: '请选择游戏Data目录', trigger: 'blur' }
  ],
  mods_store_path: [
    { required: true, message: '请选择Mod存档目录', trigger: 'blur' }
  ],
  mods_temp_cache_path: [
    { required: true, message: '请选择临时缓存目录', trigger: 'blur' }
  ],
})

const quicklyChatFormRules = reactive({
  shortcut: [
    { required: true, message: '请绑定快捷键', trigger: 'blur' }
  ],
  typing_interval: [
    { required: true, message: '请输入文字输入间隔，单位ms', trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)$/,
      message: '必须是正整数（建议 10）',
      trigger: 'blur',
    }
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
})

const keyboardFormRules = reactive({
  shortcut: [
    { required: true, message: '请绑定快捷键', trigger: 'blur' }
  ],
  typing_interval: [
    { required: true, message: '请输入文字输入间隔，单位ms', trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)$/,
      message: '必须是正整数（建议 10）',
      trigger: 'blur',
    }
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
    const store = await Store.load('config.json', { autoSave: false })

    const storeGameMod = await store.get('game_mod');
    console.log(storeGameMod);
    if (storeGameMod) {
      for (const [key, value] of Object.entries(gameMod)) {
        gameMod[key] = storeGameMod[key] || value;
      }
    }

    const storeKeyboard = await store.get('keyboard');
    console.log(storeKeyboard);
    if (storeKeyboard) {
      for (const [key, value] of Object.entries(keyboard)) {
        keyboard[key] = storeKeyboard[key] || value;
      }
    }


    const storeQuicklyChat = await store.get('quickly_chat');
    console.log(storeQuicklyChat);

    if (storeQuicklyChat) {
      for (const [key, value] of Object.entries(quicklyChat)) {
        quicklyChat[key] = storeQuicklyChat[key] || value;
      }
    }

  } catch (error) {
    console.error('获取配置文件失败:', error);
    ElMessage.error(error || '获取配置文件失败')

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
    gameMod.game_data_path = selectedPath;
  }

}

const onModsSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: "请选择Mod存档目录",
  });
  if (selectedPath) {
    gameMod.mods_store_path = selectedPath;
  }

}

const onModsTempSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: "请选择临时缓存目录",
  });
  if (selectedPath) {
    gameMod.mods_temp_cache_path = selectedPath;
  }

}


const onSubmit = async (formEl, formType) => {
  if (!formEl) return
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      console.log(valid);
      console.log(formType);
      // return;
      try {

        const store = await Store.load('config.json', { autoSave: false })

        if (formType == 'game_mod') {
          await store.set('game_mod', gameMod);

        } else if (formType == 'keyboard') {
          await store.set('keyboard', keyboard);
          // 关闭窗口是为了让最新配置生效
          await invoke('close_webview_window', { label: 'keyboard' });

        } else if (formType == 'quickly_chat') {
          await store.set('quickly_chat', quicklyChat);
          // 关闭窗口是为了让最新配置生效
          await invoke('close_webview_window', { label: 'quickly-chat' });

        }

        await store.save();

        ElMessage({
          message: '保存成功',
          type: 'success',
        })

      } catch (error) {
        console.error('配置文件保存失败:', error);
        ElMessage.error(error || '配置文件保存失败')

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
