<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">{{ $t('setting.basic.title') }}</div>
        </el-col>
      </el-row>

      <el-form :model="gameMod" label-width="165px" style="max-width: 900px" label-position="right" ref="gameModFormRef"
        :rules="basicFormRules">
        <el-form-item :label="$t('setting.basic.appConfigPath')" prop="app_config_path">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.app_config_path" disabled />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.app_config_path)">
              {{ $t('setting.operation.open') }}
            </el-button>
          </el-col>
        </el-form-item>

        <el-form-item :label="$t('setting.basic.gameDataPath')" prop="game_data_path">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.game_data_path" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onSelect">{{ $t('setting.operation.select') }}</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.game_data_path)">
              {{ $t('setting.operation.open') }}
            </el-button>
          </el-col>
        </el-form-item>

        <el-form-item :label="$t('setting.basic.modsStorePath')" prop="mods_store_path">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.mods_store_path" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onModsSelect">{{ $t('setting.operation.select') }}</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.mods_store_path)">
              {{ $t('setting.operation.open') }}
            </el-button>
          </el-col>
        </el-form-item>

        <el-form-item :label="$t('setting.basic.modsTempCachePath')" prop="mods_temp_cache_path">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.mods_temp_cache_path" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onModsTempSelect">{{ $t('setting.operation.select') }}</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.mods_temp_cache_path)">
              {{ $t('setting.operation.open') }}
            </el-button>
          </el-col>
        </el-form-item>

        <el-form-item :label="$t('setting.basic.modsInstallPriority')" prop="mods_install_priority" required>
          <el-col :span="22">
            <el-radio-group v-model="gameMod.mods_install_priority">
              <el-radio value="asc" size="large">{{ $t('setting.basic.asc') }}</el-radio>
              <el-radio value="desc" size="large">{{ $t('setting.basic.desc') }}</el-radio>
            </el-radio-group>
          </el-col>
        </el-form-item>

        <el-form-item :label="$t('setting.basic.modsUnzipType')" prop="mods_unzip_type">
          <el-col :span="22">
            <el-radio-group v-model="gameMod.mods_unzip_type" @change="unzipTypeChange">
              <el-radio :value="0" size="large">{{ $t('setting.basic.builtInPlugin') }}</el-radio>
              <el-radio :value="1" size="large">{{ $t('setting.basic.sevenZip') }}</el-radio>
            </el-radio-group>
          </el-col>
        </el-form-item>

        <el-form-item :label="$t('setting.basic.sevenZipPath')" required v-show="gameMod.mods_unzip_type == 1">
          <el-col :span="18">
            <el-input v-model.trim="gameMod.seven_zip_path" />
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="onSevenZipSelect">{{ $t('setting.operation.select') }}</el-button>
          </el-col>
          <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(gameMod.seven_zip_path)">
              {{ $t('setting.operation.open') }}
            </el-button>
          </el-col>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="onSubmit(gameModFormRef, 'game_mod')">
            {{ $t('setting.operation.save') }}
          </el-button>
          <!-- <el-button @click="resetSubmit(gameModFormRef)">重置</el-button> -->
        </el-form-item>
      </el-form>

      <el-row>
        <el-col :span="24">
          <div class="title">{{ $t('setting.language.title') }}</div>
        </el-col>
      </el-row>

      <el-form label-width="165px" style="max-width: 760px" label-position="right">
        <el-form-item :label="$t('setting.language.label')" required>
          <el-col :span="18">
            <el-select v-model="currentLang" :placeholder="$t('setting.language.placeholder')" @change="langChange">
              <el-option v-for="(item, index) in languages" :key="index" :label="item.name" :value="item.code" />
            </el-select>
          </el-col>
        </el-form-item>
      </el-form>

      <el-row>
        <el-col :span="24">
          <div class="title">{{ $t('setting.keyboard.title') }}</div>
        </el-col>
      </el-row>

      <el-form :model="keyboard" label-width="165px" style="max-width: 760px" label-position="right"
        ref="keyboardFormRef" :rules="keyboardFormRules">
        <el-form-item :label="$t('setting.keyboard.flag')" required prop="flag">
          <el-col :span="2">
            <el-switch v-model="keyboard.flag" inline-prompt
              style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949" active-text="Y" inactive-text="N" />
          </el-col>
          <el-col :span="10">
            <text style="color: red; font-size: 12px;">{{ $t('setting.keyboard.flagTip') }}</text>
          </el-col>
        </el-form-item>
        <el-form-item :label="$t('setting.keyboard.shortcut')" prop="shortcut">
          <el-col :span="18">
            <el-input v-model.trim="keyboard.shortcut" disabled />
          </el-col>
          <!-- <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.configDir)">打开</el-button>
          </el-col> -->
        </el-form-item>
        <el-form-item :label="$t('setting.keyboard.preInterval')" prop="pre_interval">
          <el-col :span="18">
            <el-input v-model.trim="keyboard.pre_interval" clearable inputmode="numeric" />
          </el-col>
        </el-form-item>
        <el-form-item :label="$t('setting.keyboard.typingInterval')" prop="typing_interval">
          <el-col :span="18">
            <el-input v-model.trim="keyboard.typing_interval" clearable inputmode="numeric" />
          </el-col>
        </el-form-item>
        <el-form-item :label="$t('setting.keyboard.enterInterval')" prop="enter_interval">
          <el-col :span="18">
            <el-input v-model.trim="keyboard.enter_interval" clearable inputmode="numeric" />
          </el-col>
        </el-form-item>
        <el-form-item :label="$t('setting.keyboard.widthAndHeight')" required>
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

        <el-form-item :label="$t('setting.keyboard.positionXAndY')" required>
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
          <el-button type="primary" @click="onSubmit(keyboardFormRef, 'keyboard')">
            {{ $t('setting.operation.save') }}
          </el-button>
          <el-button @click="resetSubmit(keyboardFormRef)">{{ $t('setting.operation.reset') }}</el-button>

        </el-form-item>
      </el-form>

      <el-row>
        <el-col :span="24">
          <div class="title">{{ $t('setting.quicklyChat.title') }}</div>
        </el-col>
      </el-row>

      <el-form :model="quicklyChat" label-width="165px" style="max-width: 760px" label-position="right"
        ref="quicklyChatFormRef" :rules="quicklyChatFormRules">
        <el-form-item :label="$t('setting.quicklyChat.flag')" required prop="flag">
          <el-col :span="2">
            <el-switch v-model="quicklyChat.flag" inline-prompt
              style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949" active-text="Y" inactive-text="N" />
          </el-col>
          <el-col :span="10">
            <text style="color: red; font-size: 12px;">{{ $t('setting.quicklyChat.flagTip') }}</text>
          </el-col>
        </el-form-item>
        <el-form-item :label="$t('setting.quicklyChat.shortcut')" prop="shortcut">
          <el-col :span="18">
            <el-input v-model.trim="quicklyChat.shortcut" disabled />
          </el-col>
          <!-- <el-col :span="2" :offset="1">
            <el-button type="primary" @click="openDir(form.configDir)">打开</el-button>
          </el-col> -->
        </el-form-item>
        <el-form-item :label="$t('setting.quicklyChat.preInterval')" prop="pre_interval">
          <el-col :span="18">
            <el-input v-model.trim="quicklyChat.pre_interval" clearable inputmode="numeric" />
          </el-col>
        </el-form-item>
        <el-form-item :label="$t('setting.quicklyChat.typingInterval')" prop="typing_interval">
          <el-col :span="18">
            <el-input v-model.trim="quicklyChat.typing_interval" clearable inputmode="numeric" />
          </el-col>
        </el-form-item>
        <el-form-item :label="$t('setting.quicklyChat.enterInterval')" prop="enter_interval">
          <el-col :span="18">
            <el-input v-model.trim="quicklyChat.enter_interval" clearable inputmode="numeric" />
          </el-col>
        </el-form-item>
        <el-form-item :label="$t('setting.quicklyChat.widthAndHeight')" required>
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

        <el-form-item :label="$t('setting.quicklyChat.phrase', { no: index + 1 })" v-for="(key, index) in chatKeys"
          :key="index" :prop="`chat.${key}`">
          <el-col :span="18">
            <el-input v-model.trim="quicklyChat['chat'][key]" clearable />
          </el-col>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="onSubmit(quicklyChatFormRef, 'quickly_chat')">
            {{ $t('setting.operation.save') }}
          </el-button>
          <el-button @click="resetSubmit(quicklyChatFormRef)">{{ $t('setting.operation.reset') }}</el-button>
        </el-form-item>
      </el-form>

    </div>
  </el-scrollbar>

</template>

<script setup>
import { ref, reactive, onMounted } from 'vue';
import { open, save } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { Store } from '@tauri-apps/plugin-store'
import { useI18n } from 'vue-i18n';
const { t, locale } = useI18n();

const gameModFormRef = ref();
const keyboardFormRef = ref();
const quicklyChatFormRef = ref();

const currentLang = ref(locale.value);
const languages = [
  { code: 'en-US', name: 'English' },
  { code: 'zh-CN', name: '简体中文' }
];

function langChange(lang) {
  // console.log(lang, currentLang.value);
  locale.value = currentLang.value;
  localStorage.setItem('Language', currentLang.value);
}

const gameMod = reactive({
  app_config_path: '',
  game_data_path: '',
  mods_store_path: '',
  mods_temp_cache_path: '',
  mods_install_priority: 'asc',
  mods_unzip_type: 0,
  seven_zip_path: "",
})

const keyboard = reactive({
  pre_interval: '250',
  typing_interval: '10',
  enter_interval: '50',
  flag: true,
  shortcut: 'ctrl + space',
  width: '340.0',
  height: '40.0',
  x: '1335.0',
  y: '960.0',
})

const quicklyChat = reactive({
  pre_interval: '250',
  typing_interval: '10',
  enter_interval: '50',
  flag: true,
  shortcut: "ctrl + .",
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


const basicFormRules = reactive({
  app_config_path: [
    { required: true, message: t('setting.basicFormRules.appConfigPathTip'), trigger: 'blur' }
  ],
  game_data_path: [
    { required: true, message: t('setting.basicFormRules.gameDataPathTip'), trigger: 'blur' }
  ],
  mods_store_path: [
    { required: true, message: t('setting.basicFormRules.modsStorePathTip'), trigger: 'blur' }
  ],
  mods_temp_cache_path: [
    { required: true, message: t('setting.basicFormRules.modsTempCachePathTip'), trigger: 'blur' }
  ],
  mods_unzip_type: [
    { required: true, message: t('setting.basicFormRules.modsUnzipTypeTip'), trigger: 'blur' }
  ],
})

const quicklyChatFormRules = reactive({
  shortcut: [
    { required: true, message: t('setting.quicklyChatFormRules.shortcutTip'), trigger: 'blur' }
  ],
  pre_interval: [
    { required: true, message: t('setting.quicklyChatFormRules.preIntervalTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)$/,
      message: t('setting.quicklyChatFormRules.preIntervalTip2'),
      trigger: 'blur',
    }
  ],
  typing_interval: [
    { required: true, message: t('setting.quicklyChatFormRules.typingIntervalTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)$/,
      message: t('setting.quicklyChatFormRules.typingIntervalTip2'),
      trigger: 'blur',
    }
  ],
  enter_interval: [
    { required: true, message: t('setting.quicklyChatFormRules.enterIntervalTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)$/,
      message: t('setting.quicklyChatFormRules.enterIntervalTip2'),
      trigger: 'blur',
    }
  ],
  width: [
    { required: true, message: t('setting.quicklyChatFormRules.widthTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: t('setting.quicklyChatFormRules.widthTip2'),
      trigger: 'blur',
    }
  ],
  height: [
    { required: true, message: t('setting.quicklyChatFormRules.heightTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: t('setting.quicklyChatFormRules.heightTip2'),
      trigger: 'blur',
    }
  ],
})

const keyboardFormRules = reactive({
  shortcut: [
    { required: true, message: t('setting.keyboardFormRules.shortcutTip'), trigger: 'blur' }
  ],
  pre_interval: [
    { required: true, message: t('setting.keyboardFormRules.preIntervalTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)$/,
      message: t('setting.keyboardFormRules.preIntervalTip2'),
      trigger: 'blur',
    }
  ],
  typing_interval: [
    { required: true, message: t('setting.keyboardFormRules.typingIntervalTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)$/,
      message: t('setting.keyboardFormRules.typingIntervalTip2'),
      trigger: 'blur',
    }
  ],
  enter_interval: [
    { required: true, message: t('setting.keyboardFormRules.enterIntervalTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)$/,
      message: t('setting.keyboardFormRules.enterIntervalTip2'),
      trigger: 'blur',
    }
  ],
  width: [
    { required: true, message: t('setting.keyboardFormRules.widthTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: t('setting.keyboardFormRules.widthTip2'),
      trigger: 'blur',
    }
  ],
  height: [
    { required: true, message: t('setting.keyboardFormRules.heightTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: t('setting.keyboardFormRules.heightTip2'),
      trigger: 'blur',
    }
  ],
  x: [
    { required: true, message: t('setting.keyboardFormRules.positionXTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: t('setting.keyboardFormRules.positionXTip2'),
      trigger: 'blur',
    }
  ],
  y: [
    { required: true, message: t('setting.keyboardFormRules.positionYTip1'), trigger: 'blur' },
    {
      pattern: /^(0|[1-9]\d*)\.\d$/,
      message: t('setting.keyboardFormRules.positionYTip2'),
      trigger: 'blur',
    }
  ],
})

onMounted(async () => {
  const loading = ElLoading.service({
    lock: true,
    text: t('setting.init.loadingText'),
    background: 'rgba(0, 0, 0, 0.7)',
  })
  try {
    const store = await Store.load('config.json', { autoSave: false })

    const storeGameMod = await store.get('game_mod');
    // console.log(storeGameMod);
    if (storeGameMod) {
      for (const [key, value] of Object.entries(gameMod)) {
        gameMod[key] = storeGameMod[key] || gameMod[key];
      }
    }
    // console.log(gameMod);

    const storeKeyboard = await store.get('keyboard');
    // console.log(storeKeyboard);
    if (storeKeyboard) {
      for (const [key, value] of Object.entries(keyboard)) {
        keyboard[key] = storeKeyboard[key] || keyboard[key];
      }
    }


    const storeQuicklyChat = await store.get('quickly_chat');
    // console.log(storeQuicklyChat);
    if (storeQuicklyChat) {
      for (const [key, value] of Object.entries(quicklyChat)) {
        quicklyChat[key] = storeQuicklyChat[key] || quicklyChat[key];
      }
    }

  } catch (error) {
    console.error('获取配置文件失败:', error);
    ElMessage.error(error || t('setting.init.fail'))

  } finally {
    loading.close();

  }

});

async function unzipTypeChange() {
  if (gameMod.mods_unzip_type == 1) {
    gameMod.seven_zip_path = await getSevenZipPath();
  }
}

async function openDir(path) {
  if (!path) {
    ElMessage.error(t('setting.openDir.noPathError'))
    return;
  }
  try {
    await invoke('open_folder', { path });
    // console.log('Folder opened successfully!');
  } catch (error) {
    console.error('Failed to open folder:', error);
    ElMessage.error(error || t('setting.openDir.fail'))

  }
}

async function getSevenZipPath() {
  let path = '';
  try {
    path = await invoke('get_seven_zip_path');
  } catch (error) {

  }

  return path
}

const onSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: t('setting.selectGameDataPathTitle'),
  });
  if (selectedPath) {
    gameMod.game_data_path = selectedPath;
  }

}

const onModsSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: t('setting.selectModsStorePathTitle'),
  });
  if (selectedPath) {
    gameMod.mods_store_path = selectedPath;
  }

}

const onModsTempSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: t('setting.selectModsTempCachePathTitle'),
  });
  if (selectedPath) {
    gameMod.mods_temp_cache_path = selectedPath;
  }

}

const onSevenZipSelect = async () => {
  const selectedPath = await open({
    multiple: false,
    directory: true,
    title: t('setting.selectSevenZipPathTitle'),
  });
  if (selectedPath) {
    gameMod.seven_zip_path = selectedPath;
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
          if (gameMod.mods_unzip_type == 1 && !gameMod.seven_zip_path) {
            ElMessage.error(t('setting.selectSevenZipPathTitle'))
            return;
          }
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
          message: t('setting.operation.success'),
          type: 'success',
        })

      } catch (error) {
        console.error('配置文件保存失败:', error);
        ElMessage.error(error || t('setting.operation.fail'))

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
