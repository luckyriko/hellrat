<template>
  <div class="container" @click="displayNoneMenu">
    <!-- 工具栏 -->
    <div class="header">
      <el-popconfirm title="请选择一个吧？！" @confirm="openUploadModWindow" @cancel="getModsRecords" placement="bottom"
        confirm-button-text="新增上传" cancel-button-text="选择现有" :width="250">
        <template #reference>
          <el-button type="warning" :icon="DocumentAdd" plain>添加</el-button>
        </template>
      </el-popconfirm>
      <el-button type="info" :icon="Operation" plain @click="toggleView">{{ listShowType == 'grid' ? '图标' : '列表'
      }}</el-button>
      <el-button type="success" :icon="Sort" plain @click="searchModByOrder">{{ orderAscFlag ? '正序' : '倒序'
      }}</el-button>


      <div style="margin-left: 20px;">
        <el-select v-model="environment" value-key="id" placeholder="Select" style="width: 240px"
          @change="environmentChange">
          <el-option v-for="item in environmentList" :key="item.id" :label="item.name" :value="item" />
        </el-select>

        <el-button type="primary" text @click="addEnvVisible = true" style="padding-left: 6px;">
          <template #icon>
            <el-icon size="20" color="#666">
              <EditPen />
            </el-icon>
          </template>
        </el-button>
      </div>

      <div style="margin-left: 20px;margin-right: 20px;flex:1">
        <el-input v-model="search" placeholder="请输入名称" clearable @keyup.enter.native="searchMod">
          <template #append>
            <el-button @click="searchMod"><i-ep-search /></el-button>
          </template>
        </el-input>
      </div>

      <el-button type="primary" :icon="MagicStick" plain @click="installMods">应用</el-button>
      <el-button type="danger" :icon="Delete" plain @click="uninstallMods">清除</el-button>
    </div>
    <!-- 工具栏. -->

    <!-- 表格 -->
    <el-scrollbar class="main-scroll">
      <div v-show="listShowType == 'grid'" class="content-area grid" id="sortableGrid">
        <div v-for="item in modsData" :key="item.id" class="item" :class="{ 'install': item.env_mod_install_flag }"
          @contextmenu.prevent="openMenu($event, item)">
          <el-image :src="item.previewPath" fit="cover" style="width: 100%; height: 200px"
            @click="openModInfoDialog(item.id)">
            <template #error>
              <div class="image-slot">
                <el-icon><i-ep-picture /></el-icon>
              </div>
            </template>
          </el-image>
          <div class="text" @click="openSelectOptionsDialog(item.id, item.type)">
            <span>{{ item.name }}</span>
            <el-icon size="20" color="violet" v-if="item.type == 2"><i-ep-edit /></el-icon>
          </div>
          <div class="switch">
            <el-switch v-model="item.activate" style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949"
              :active-value="1" :inactive-value="0" @change="changeModActivate(item)" />
          </div>
        </div>
      </div>
      <div v-show="listShowType != 'grid'" class="content-area list" id="sortableList">
        <div v-for="item in modsData" :key="item.id" class="item">
          <el-switch v-model="item.activate" style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949"
            :active-value="1" :inactive-value="0" @change="changeModActivate(item)" />
          <el-image :src="item.previewPath" fit="cover" @click="openModInfoDialog(item.id)"
            style="width: 40px; height: 40px; border-radius: 50%;margin-right: 10px;margin-left: 10px;"
            :class="{ 'install': item.env_mod_install_flag }">
            <template #error>
              <div class="image-slot">
                {{ item.name[0].toUpperCase() }}
              </div>
            </template>
          </el-image>
          <div style="display: flex;flex:1; flex-direction: row;justify-content: space-between;">
            <div class="text" @click="openModInfoDialog(item.id)">{{ item.name }}</div>
            <div>
              <el-button size="small" plain @click="openDir(item)">
                打开
              </el-button>
              <el-button size="small" plain @click="openModInfoDialog(item.id)">
                详情
              </el-button>
              <el-button size="small" plain @click="handleEdit(item)">
                编辑
              </el-button>
              <el-button type="danger" size="small" plain @click="deleteMod(item)">
                删除
              </el-button>
            </div>
          </div>
        </div>
      </div>
    </el-scrollbar>

    <!-- 表格. -->

    <!-- 右键功能 -->
    <Menu ref="menuRef" :mentList="mentList" @select-label="selectMenuLabel"></Menu>
    <!-- 右键功能. -->

    <div class="footer">
      <div class="text">
        <!-- <div>环境Mod总数：1000</div>
        <div>Mod总数：1000</div> -->
        <el-button plain color="#626aef" @click="activateAllMods" size="small">全启</el-button>
        <el-button plain color="#626aef" @click="disableAllMods" size="small">全禁</el-button>
        <el-button plain color="#626aef" @click="deleteAllMods" size="small">全删</el-button>
      </div>
      <div class="text">
        <div @click="donateFlag = !donateFlag">
          <span v-show="donateFlag" style="color: #666;font-size: 0.8rem;">如果你认可开发者的辛勤劳作 请他喝杯饮料吧</span>
        </div>
        <el-icon color="deepskyblue" size="20" @click="donate">
          <ColdDrink />
        </el-icon>
      </div>
    </div>

  </div>

  <SelectOptions v-model="dialogVisible" @close="closeDialog" :id="currentSelectId" :env-id="environment.id"
    :mods-store-path="gameMod.mods_store_path"></SelectOptions>

  <AddEnvironments v-model="addEnvVisible" @close="closeEnvVisible" :list="environmentList"
    @refresh="getEnvironmentList">
  </AddEnvironments>

  <SelectMods v-model="selectModsVisible" @close="closeSelectModsVisible" :env-id="environment.id"
    :list="modsRecordsList" @refresh="getModsList">
  </SelectMods>

  <ShowModInfo v-model="showModInfoVisible" @close="closeShowModInfoVisible" :env-id="environment.id"
    :record-id="showModId" :mods-store-path="gameMod.mods_store_path">
  </ShowModInfo>

  <el-dialog v-model="dialogFormVisible" title="修改Mod信息" width="40%" :close-on-click-modal="false">
    <el-form ref="formRef" :model="form" :rules="rules">
      <el-form-item label="名称" :label-width="formLabelWidth" prop="name">
        <el-input v-model="form.name" autocomplete="off" />
      </el-form-item>
      <!-- <el-form-item label="类型" :label-width="formLabelWidth" prop="mod_type">
        <el-radio-group v-model="form.mod_type">
          <el-radio value="model">模型</el-radio>
          <el-radio value="voice">音频/杂项</el-radio>
        </el-radio-group>
      </el-form-item> -->
      <el-form-item label="图标" :label-width="formLabelWidth" prop="icon">
        <div v-if="form.previewPath" class="image" @click="selectPreViewImg">
          <el-image style="height: 150px; background-color: #f0f0f0;" :src="form.previewPath" fit="fill" />
          <el-icon class="close-icon" size="30" color="red"
            @click.stop="removeImg"><i-ep-circle-close-filled /></el-icon>
        </div>
        <div v-else class="image" @click="selectPreViewImg">
          <el-icon size="20" color="#666"><i-ep-plus /></el-icon>
        </div>
        <el-input v-model.trim="form.icon" type="hidden" />
      </el-form-item>
      <el-form-item label="作者" :label-width="formLabelWidth" prop="author">
        <el-input v-model="form.author" autocomplete="off" />
      </el-form-item>
      <el-form-item label="版本" :label-width="formLabelWidth" prop="version">
        <el-input v-model="form.version" autocomplete="off" />
      </el-form-item>
      <el-form-item label="链接" :label-width="formLabelWidth" prop="link">
        <el-input v-model="form.link" autocomplete="off" />
      </el-form-item>
      <el-form-item label="详情" :label-width="formLabelWidth" prop="desc">
        <el-input v-model="form.desc" autocomplete="off" type="textarea" />
      </el-form-item>
      <el-form-item label="图片路径" :label-width="formLabelWidth" prop="preview" style="display: none;">
        <el-input v-model="form.preview" autocomplete="off" disabled />
      </el-form-item>
      <el-form-item label="图片网址" :label-width="formLabelWidth" prop="previewPath" style="display: none;">
        <el-input v-model="form.previewPath" autocomplete="off" disabled />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="dialogFormVisible = false">取消</el-button>
        <el-button @click="resetForm(formRef)">重置</el-button>
        <el-button type="primary" :loading="loadingFlag" @click="onSubmit(formRef)">
          确认
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<script setup>
import { h, ref, reactive, onMounted, toRaw, computed, nextTick } from 'vue';
import { open as openShell } from '@tauri-apps/plugin-shell';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { exists, mkdir, create, readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';
import SelectOptions from './components/SelectOptions.vue';
import AddEnvironments from './components/AddEnvironments.vue';
import SelectMods from './components/SelectMods.vue';
import ShowModInfo from './components/ShowModInfo.vue';
import Menu from './components/Menu.vue';
import Sortable from 'sortablejs';
import { DocumentAdd, Delete, EditPen, Operation, Sort, MagicStick, ColdDrink } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-shell';
import { listen } from '@tauri-apps/api/event';

const donateFlag = ref(true);
const donate = async () => {
  await open("https://afdian.com/a/luckyriko");

}

const gameMod = reactive({
  app_config_path: '',
  game_data_path: '',
  mods_store_path: '',
  mods_temp_cache_path: ''
});

const environment = ref({
  id: 1,
  name: 'default',
  activate: 1
});

const environmentList = ref([]);

const environmentChange = async (item) => {
  // console.log(item.id, item.name);
  try {
    // console.log(environment.value);
    invoke("update_environment_activate", { id: item.id }).then(async () => {
      await getEnvironmentList();
      await getModsList();
    });
  } catch (error) {
    ElMessage.error("切换环境失败，请刷新页面重试！")

  }


}

const toggleView = () => {
  listShowType.value = listShowType.value == 'grid' ? 'list' : 'grid';
  localStorage.setItem("ListShowType", listShowType.value);
}

// 切换该环境下的Mod安装状态
const changeModActivate = async (mod) => {
  // console.log(mod.activate, Number(mod.activate));
  try {
    await invoke("update_environment_mod_activate", { id: mod.env_mod_id, activate: Number(mod.activate) });
  } catch (error) {
    ElMessage.error("切换失败，请刷新页面重试！")

  }
}

const activateAllMods = async () => {
  const loading = ElLoading.service({
    lock: true,
    text: '正在启用游戏补丁ing...',
    background: 'rgba(0, 0, 0, 0.7)',
  })
  try {
    await Promise.allSettled(modsData.value.map(async mod => {
      await invoke("update_environment_mod_activate", { id: mod.env_mod_id, activate: 1 });
    }));

  } catch (error) {
    ElMessage.error("全启失败：" + String(error))

  } finally {
    loading.close();
    await getModsList();

  }
}

const disableAllMods = async () => {
  const loading = ElLoading.service({
    lock: true,
    text: '正在禁用游戏补丁ing...',
    background: 'rgba(0, 0, 0, 0.7)',
  })
  try {
    await Promise.allSettled(modsData.value.map(async mod => {
      await invoke("update_environment_mod_activate", { id: mod.env_mod_id, activate: 0 });
    }));
  } catch (error) {
    ElMessage.error("全禁失败：" + String(error))

  } finally {
    loading.close();
    await getModsList();

  }
}

const deleteAllMods = async () => {
  if (!gameMod.mods_store_path) {
    ElMessage.error('未获取到Mod存档目录');
    return
  }

  if (!environment.value.id) {
    ElMessage.error('未获取到环境变量Id');
    return
  }

  const delete_file_flag = ref(false);
  ElMessageBox({
    title: '确认删除该环境下的所有Mod记录吗？',
    message: () =>
      h('div', null, [
        h('span', null, '我还需要删除文件，删除前请自行备份。'),
        h(ElSwitch, {
          modelValue: delete_file_flag.value,
          'onUpdate:modelValue': (val) => {
            delete_file_flag.value = val
          },
        }),
      ]),
    showCancelButton: true,
    confirmButtonText: '确定',
    cancelButtonText: '取消',
  }).then(async (action) => {
    if (action == 'confirm') {
      const loading = ElLoading.service({
        lock: true,
        text: '正在删除游戏补丁ing...',
        background: 'rgba(0, 0, 0, 0.7)',
      })

      try {
        const results = await Promise.allSettled(modsData.value.map(async mod => {
          await invoke("delete_one_mod", {
            record_id: mod.id,
            env_id: Number(environment.value.id),
            delete_file_flag: delete_file_flag.value ? 1 : 0,
            mod_dir_name: mod.path
          });
        }));

        const failed = results
          .filter(r => r.status === "rejected")
          .map(r => r.reason);

        console.log("失败的结果:", failed);

        if (failed.length == 0) {
          ElMessage({
            message: '删除成功',
            type: 'success'
          })
        } else {
          const uniqueFailed = [...new Set(failed)];
          ElMessage.error('删除失败：共' + failed.length + '个错误：' + uniqueFailed.join(''))

        }

      } catch (error) {
        console.error("Error delete mod:", error);
        ElMessage.error('删除失败：' + String(error))

      } finally {
        loading.close();
        await getModsList();

      }
    }
  }).catch(() => {
    ElMessage({
      type: 'info',
      message: '已取消',
    })
  })
}


function setGridSort() {
  const el = document.getElementById('sortableGrid');
  new Sortable(el, {
    sort: true,
    animation: 150,
    swapThreshold: 0.5,

    // handle: '.handle-drag',
    ghostClass: 'sortable-ghost',
    // easing: 'cubic-bezier(1, 0, 0, 1)',
    onStart: (e) => {
      // console.log('onstart:', e);
    },
    onEnd: (e) => {
      // console.log('onEnd:', e)

      // 位置没变化直接返回
      if (e.oldIndex == e.newIndex) {
        console.log('排序无变化');
        return;
      }

      // 位置有变化则更新源数组对象为排序后的状况
      const currRow = modsData.value.splice(e.oldIndex, 1)[0];
      modsData.value.splice(e.newIndex, 0, currRow);

      // 打印值与页面进行对比查看是否正确
      // console.log(modsData.value);

      // 更新后端排序值
      updateEnvModSort(Number(e.newIndex))

    },
  })
}


function setListSort() {
  const el = document.getElementById('sortableList');
  new Sortable(el, {
    sort: true,
    animation: 150,
    swapThreshold: 0.5,

    // handle: '.handle-drag',
    ghostClass: 'sortable-ghost',
    // easing: 'cubic-bezier(1, 0, 0, 1)',
    onStart: (e) => {
      // console.log('onstart:', e);
    },
    onEnd: (e) => {
      // console.log('onEnd:', e)

      // 位置没变化直接返回
      if (e.oldIndex == e.newIndex) {
        console.log('排序无变化');
        return;
      }

      // 位置有变化则更新源数组对象为排序后的状况
      const currRow = modsData.value.splice(e.oldIndex, 1)[0];
      modsData.value.splice(e.newIndex, 0, currRow);

      // 打印值与页面进行对比查看是否正确
      // console.log(modsData.value);

      // 更新后端排序值
      updateEnvModSort(Number(e.newIndex))

    },
  })
}

const updateEnvModSort = async (index) => {
  let env_mod_id = modsData.value[index]['env_mod_id'];
  let pre_sort = '', next_sort = '';
  if (orderAscFlag.value) {
    // 正序
    // pre_sort取前一个元素的排序值, next_sort取后一个元素的排序值
    // 如果是第一个则pre_sort为空, 如果是最后一个则next_sort为空
    if (index == 0) {
      pre_sort = '';
      next_sort = modsData.value[index + 1]['sort'] ?? '';
    }

    if (index == modsData.value.length - 1) {
      pre_sort = modsData.value[index - 1]['sort'] ?? '';
      next_sort = '';
    }

    if (index > 0 && index < modsData.value.length - 1) {
      pre_sort = modsData.value[index - 1]['sort'] ?? '';
      next_sort = modsData.value[index + 1]['sort'] ?? '';
    }

  } else {
    // 倒序
    if (index == 0) {
      pre_sort = modsData.value[index + 1]['sort'] ?? '';
      next_sort = '';
    }

    if (index == modsData.value.length - 1) {
      pre_sort = '';
      next_sort = modsData.value[index - 1]['sort'] ?? '';
    }

    if (index > 0 && index < modsData.value.length - 1) {
      pre_sort = modsData.value[index + 1]['sort'] ?? '';
      next_sort = modsData.value[index - 1]['sort'] ?? '';
    }
  }
  // console.log(`当前修改的id：${env_mod_id}, pre_sort：${pre_sort}, next_sort：${next_sort}`);

  try {
    let sort = await invoke("update_environment_mod_sort", { env_mod_id, pre_sort, next_sort });
    modsData.value[index]['sort'] = sort;
    // ElMessage({
    //   message: '排序值更新成功',
    //   type: 'success',
    // })
  } catch (error) {
    console.error("安装失败", error);
    ElMessage.error('排序值更新失败，请手动刷新后重试！')

  }
}

const mentList = [
  {
    label: '打开',
    operation: 'openDir'
  },
  {
    label: '编辑',
    operation: 'handleEdit'
  },
  {
    label: '删除',
    operation: 'deleteMod'
  },
];

// 菜单dom
const menuRef = ref();
const currentMenuData = ref(null);


//点击任何区域，隐藏菜单
const displayNoneMenu = () => {
  if (menuRef.value) {
    menuRef.value.show = false;
  }
  currentMenuData.value = null;
}
//右键事件
const openMenu = (event, row) => {
  //阻止浏览器默认事件
  event.preventDefault();
  // console.log(event.clientX, event.clientY);
  // 通过获取到的组件实例，设置菜单的显示和位置
  menuRef.value.show = true;
  menuRef.value.setPosition(event.clientX, event.clientY);
  currentMenuData.value = row
}

const selectMenuLabel = (menuItem) => {
  // console.log(currentMenuData.value, menuItem.operation);
  switch (menuItem.operation) {
    case 'openDir':
      openDir(currentMenuData.value);
      break;
    case 'handleEdit':
      handleEdit(currentMenuData.value);
      break;
    case 'deleteMod':
      deleteMod(currentMenuData.value);
      break;
    default:
      console.log('Unknown operation');
  }

}

const addEnvVisible = ref(false);
const closeEnvVisible = () => {
  addEnvVisible.value = false;
};


const showModInfoVisible = ref(false);
const showModId = ref(0);
const closeShowModInfoVisible = () => {
  showModInfoVisible.value = false;
};
const openModInfoDialog = async (id) => {
  showModId.value = id;
  showModInfoVisible.value = true;
}

// 排序 
// 定义一个响应式变量，用于控制对话框的显示与隐藏
const dialogVisible = ref(false);
const currentSelectId = ref(0);

// 打开对话框的函数，将 dialogVisible 设置为 true
const openSelectOptionsDialog = (id, type) => {
  if (!id) return;
  if (type != 2) return;
  currentSelectId.value = id;
  dialogVisible.value = true;
};

// 关闭对话框的函数，将 dialogVisible 设置为 false
const closeDialog = () => {
  dialogVisible.value = false;
};

// 编辑弹窗
const formRef = ref();
const dialogFormVisible = ref(false)
const formLabelWidth = '80px'
const form = reactive({
  name: '',
  icon: '',
  author: '',
  version: '',
  link: '',
  desc: '',
  previewPath: '',
  // tag: '',

});
const loadingFlag = ref(false);

const selectPreViewImg = async () => {
  const selectedPath = await openDialog({
    multiple: false,
    directory: false,
    title: "请选择预览图",
  });


  if (selectedPath) {
    form.icon = selectedPath;
    form.preview = selectedPath;
    form.previewPath = convertFileSrc(selectedPath);
  }

};
const removeImg = () => {
  form.icon = '';
  form.preview = '';
  form.previewPath = '';
};

const rules = reactive({
  name: [
    { required: true, min: 1, max: 255, message: '最少1个字符，最长255个字符', trigger: ['blur', 'change'] }
  ]
})

const handleEdit = (row) => {
  if (row) {
    Object.assign(form, row);
    dialogFormVisible.value = true;
  }
}

import { WebviewWindow } from '@tauri-apps/api/webviewWindow'


async function openUploadModWindow() {
  let label = 'upload';
  const exists = await invoke('focus_if_webview_window_exists', { label });
  if (!exists) {
    // console.log('创建新窗口');
    const webview = new WebviewWindow(label, {
      url: '/upload',
      width: 400,
      height: 600,
      title: 'Mod存档',
      dragDropEnabled: true

    });
    webview.once('tauri://created', function () {
      // webview successfully created
    });
    webview.once('tauri://error', function (e) {
      // an error happened creating the webview
      console.log(e);
    });
  } else {
    // console.log('窗口已存在，已聚焦')
  }


}

const installMods = async () => {
  const loading = ElLoading.service({
    lock: true,
    text: '正在安装游戏补丁...',
    background: 'rgba(0, 0, 0, 0.7)',
  })

  try {
    await invoke("install_mods", { env_id: Number(environment.value.id) });
    ElMessage({
      message: '安装成功',
      type: 'success',
    })
  } catch (error) {
    console.error("安装失败", error);
    ElMessage.error(error || '安装失败')

  } finally {
    loading.close();
    await getModsList();

  }
}


const onSubmit = async (formEl) => {
  if (!formEl) return;
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      loadingFlag.value = true;

      try {
        let mod = modsData.value.find(x => x.id == form.id);
        let up_img_flag = true;
        if (mod?.icon == form.icon) {
          // console.log('图片无需更新');
          up_img_flag = false;
        }

        console.log(form);

        await invoke('up_mod_info', { mod_info: form, record_dir: modsDir.value, up_img_flag });
        // console.log('up_mod_info successfully!');
        await getModsList();
        dialogFormVisible.value = false;
        ElMessage({
          message: '修改成功',
          type: 'success',
        })

      } catch (error) {
        console.error('Failed to submit:', error);
        ElMessage.error(String(error) || '修改失败')

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

// 搜索
const search = ref('');

// 存档列表
const modsData = ref([]);

// 存档列表ref
const tableRef = ref();

// mod存档目录
const modsDir = ref('');

// 游戏data目录
const gameDataDir = ref('');

const listShowType = ref('grid');

import { Store } from '@tauri-apps/plugin-store'
import { tr } from 'element-plus/es/locales.mjs';

// 挂载时获取设置参数里的各种目录
onMounted(async () => {
  try {
    await listen('save-mods-complete', (event) => {
      // console.log(event.payload);
      getModsList();
    });


    const store = await Store.load('config.json', { autoSave: false })
    const storeGameMod = await store.get('game_mod');
    // console.log(storeGameMod);
    if (storeGameMod) {
      for (const [key, value] of Object.entries(gameMod)) {
        gameMod[key] = storeGameMod[key] || value;
      }
    }

    orderAscFlag.value = Number(localStorage.getItem("OrderAscFlag")) == 1 ? true : false;
    listShowType.value = localStorage.getItem("ListShowType") ? localStorage.getItem("ListShowType") : 'grid';

    await getEnvironmentList();
    await getModsList();
    setGridSort();
    setListSort();

  } catch (error) {

  }


});


const statistics = reactive({
  model_activate_count: 0,
  model_total_count: 0,
  records_activate_count: 0,
  records_total_count: 0,
  voice_activate_count: 0,
  voice_total_count: 0
});

async function getStatistics() {
  try {
    const data = await invoke("get_statistics");
    // console.log("统计数据:", data);
    Object.assign(statistics, data);

  } catch (error) {
    console.error("获取统计数据失败", error);
  }
}

async function activeAllMods() {
  modsData.value.forEach(element => {
    element.activate = true;
  });
}

async function searchMod() {
  await getModsList(search.value);
}

async function searchModByOrder() {
  orderAscFlag.value = !orderAscFlag.value;
  localStorage.setItem("OrderAscFlag", orderAscFlag.value ? "1" : "0");

  await getModsList(search.value);
}

// 切换安装状态时触发
const activateChange = async (row) => {
  // console.log(row);

}

// 跳转到默认浏览器打开链接
const openBrowser = async (url) => {
  await openShell(url);

}

// 打开mod对应的存档目录
async function openDir(row) {
  if (!gameMod.mods_store_path || !row.path) {
    ElMessage.error('Mod存档目录未设置')
    return;
  }
  try {
    let dir_path = await join(gameMod.mods_store_path, row.path);
    await invoke('open_folder', { path: dir_path });
    // console.log('Folder opened successfully!');
  } catch (error) {
    console.error('打开Mod目录失败:', error);
    ElMessage.error(error || 'Oops, this is a error message.')

  }
}

// 获取当前环境
async function getEnvironmentList() {
  try {
    let env = await invoke('get_environment_list');
    if (env.length == 0) return;
    env.forEach(element => {
      element.editName = element.name;
      element.editFlag = false;
    });
    environmentList.value = env;
    // console.log('env: ', environmentList.value);
    let activateItem = env.find(x => x.activate == 1);
    environment.value = activateItem ? activateItem : env[0];

  } catch (error) {
    console.error('获取当前环境变量失败:', error);
    ElMessage.error(error || 'Oops, this is a error message.')

  }
}


// 一键卸载
const uninstallMods = async () => {
  if (!gameMod.game_data_path) {
    ElMessage.error('游戏data目录未配置！');
    return;
  }

  const delete_all_flag = ref(false);

  ElMessageBox({
    title: '确认删除通过本软件安装的所有游戏补丁？',
    message: () =>
      h('div', null, [
        h('span', null, '我还需要删除所有游戏补丁'),
        h(ElSwitch, {
          modelValue: delete_all_flag.value,
          'onUpdate:modelValue': (val) => {
            delete_all_flag.value = val
          },
        }),
      ]),
    showCancelButton: true,
    confirmButtonText: '确定',
    cancelButtonText: '取消',
  }).then(async (action) => {
    if (action == 'confirm') {
      const loading = ElLoading.service({
        lock: true,
        text: '游戏补丁清除中...',
        background: 'rgba(0, 0, 0, 0.7)',
      })

      try {
        await invoke("uninstall_mods", {
          all_mod_flag: delete_all_flag.value ? 1 : 0
        });
        ElMessage({
          message: '删除成功',
          type: 'success'
        })

      } catch (error) {
        console.error("Error delete mod:", error);
        ElMessage.error('删除失败：' + String(error))

      } finally {
        loading.close();
        await getModsList();

      }
    }
  }).catch(() => {
    ElMessage({
      type: 'info',
      message: '已取消',
    })
  })
}

// 删除mod存档
const deleteMod = async (row) => {
  // if (Number(row.activate) === 1) {
  //   ElMessage.error('请先禁用再删除！');
  //   return
  // }

  if (!gameMod.mods_store_path) {
    ElMessage.error('未获取到Mod存档目录');
    return
  }

  if (!environment.value.id) {
    ElMessage.error('未获取到环境变量Id');
    return
  }

  const delete_file_flag = ref(false);
  ElMessageBox({
    title: '确认删除该环境下的Mod记录吗？',
    message: () =>
      h('div', null, [
        h('span', null, '我还需要删除文件，删除前请自行备份。'),
        h(ElSwitch, {
          modelValue: delete_file_flag.value,
          'onUpdate:modelValue': (val) => {
            delete_file_flag.value = val
          },
        }),
      ]),
    showCancelButton: true,
    confirmButtonText: '确定',
    cancelButtonText: '取消',
  }).then(async (action) => {
    if (action == 'confirm') {
      const loading = ElLoading.service({
        lock: true,
        text: '删除中...',
        background: 'rgba(0, 0, 0, 0.7)',
      })

      try {
        await invoke("delete_one_mod", {
          record_id: row.id,
          env_id: Number(environment.value.id),
          delete_file_flag: delete_file_flag.value ? 1 : 0,
          mod_dir_name: row.path
        });
        ElMessage({
          message: '删除成功',
          type: 'success'
        })
        await getModsList();

      } catch (error) {
        console.error("Error delete mod:", error);
        ElMessage.error('删除失败：' + String(error))

      } finally {
        loading.close();

      }
    }
  }).catch(() => {
    ElMessage({
      type: 'info',
      message: '已取消',
    })
  })

}

// 清除过滤条件
const clearFilter = () => {
  tableRef.value.clearFilter()
}

// 过滤函数
const filterHandler = (value, row) => {
  // console.log(value, row.activate);
  return row.activate === value
}

// const type = (row, column) => {
//   return row.mod_type === 'model' ? '模型' : '音频/杂项';
// }

const orderAscFlag = ref(false);

// 获取mod存档
async function getModsList(name = '') {
  if (!name) {
    search.value = '';
  }
  const loading = ElLoading.service({
    lock: true,
    text: 'Loading',
    background: 'rgba(0, 0, 0, 0.7)',
  })

  try {
    let order_asc_flag = orderAscFlag.value ? 1 : 0;
    let mods = await invoke("get_environment_mods_records", { env_id: Number(environment.value.id), search: name, order_asc_flag });
    await Promise.allSettled(mods.map(async element => {
      let { preview, assetUrl } = await previewImg(element);
      element.preview = preview;
      element.previewPath = assetUrl;
    }));

    modsData.value = mods;
    // console.log("get mod records:", modsData.value);

  } catch (error) {
    console.error("Error get records:", error);
    ElMessage.error('获取失败')

  } finally {
    loading.close();
    // getStatistics();

  }
}

const selectModsVisible = ref(false);
const closeSelectModsVisible = () => {
  selectModsVisible.value = false;
};

const modsRecordsList = ref([]);

// 获取mod存档
async function getModsRecords() {
  try {
    modsRecordsList.value = [];
    let mods = await invoke("get_mods_records_with_env_add_flag", { env_id: Number(environment.value.id) });
    await Promise.allSettled(mods.map(async element => {
      if (element.env_add_flag != 1) {
        let { preview, assetUrl } = await previewImg(element);
        modsRecordsList.value.push({
          id: element.id,
          name: element.name,
          env_add_flag: element.env_add_flag,
          preview,
          previewPath: assetUrl
        })
      }
    }));

    if (modsRecordsList.value.length > 0) {
      selectModsVisible.value = true;
      // console.log("getModsRecordsWithEnv:", modsRecordsList.value);
    } else {
      ElMessage.error('暂无可添加Mod！')

    }
  } catch (error) {
    console.error("Error get records:", error);
    ElMessage.error('获取失败')

  } finally {


  }
}

// 预览图片
const previewImg = async (row) => {
  if (gameMod.mods_store_path && row.path && row.icon) {
    let imgPath = await join(gameMod.mods_store_path, row.path, row.icon);
    const assetUrl = convertFileSrc(imgPath);
    return {
      assetUrl,
      preview: imgPath
    }
  } else {
    return {
      assetUrl: '',
      preview: ''
    };
  }
};


</script>

<style scoped lang="scss">
// 固定屏幕高度
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  box-sizing: border-box;
  // padding: 20px 20px 0 20px;
  // position: relative;
}

// 固定头部和底部高度
.header,
.footer {
  background: #f5f5f5;
  border-bottom: 1px solid #ddd;
  display: flex;
  flex-direction: row;
  align-items: center;
  padding-left: 20px;
  padding-right: 20px;
}

.header {
  height: 50px;
  border-top: 1px solid #ddd;
  justify-content: space-between;
}

.footer {
  height: 30px;
  border-top: 1px solid #ddd;
  justify-content: space-between;
}

.text {
  display: flex;
  flex-direction: row;
  align-items: center;
}

// .content {
//   flex: 1;
//   overflow: auto;
//   background: #fff;
//   padding: 16px;
// }

// .title {
//   font-size: 20px;
//   font-weight: bold;
//   margin-bottom: 10px;
// }

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

.sortable-ghost {
  color: white;
  background-color: skyblue !important;
}

// .card-grid {
//   display: grid;
//   grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
//   gap: 20px;
//   padding: 20px;
//   // height: 100%;
//   // background-color: red;

//   .card {
//     background-color: #fff;
//     border: 1px solid #eee;
//     border-radius: 10px;
//     overflow: hidden;
//     box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
//     transition: transform 0.2s ease;
//     display: flex;
//     flex-direction: column;

//     // &:hover {
//     //   transform: translateY(-4px);
//     // }

//     img {
//       width: 100%;
//       height: 160px;
//       object-fit: cover;
//     }

//     .card-text {
//       padding: 12px;
//       font-size: 16px;
//       text-align: center;

//       h3 {
//         margin: 0;
//         font-size: 18px;
//       }

//       p {
//         font-size: 14px;
//         color: #666;
//         margin-top: 6px;
//       }
//     }
//   }
// }

.content-area {
  padding: 16px;

  .item {
    display: flex;
    flex-direction: column;
    align-items: center;
    position: relative;

    .text {
      display: flex;
      flex-direction: row;
      align-items: center;
      color: #333;
      font-size: 0.9rem;
    }
  }

  &.grid {
    display: grid;
    gap: 16px;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));

    .item {
      background: #f0f0f0;
      border-radius: 6px;
    }

    .text {
      padding: 6px;
    }

    .switch {
      position: absolute;
      top: 0;
      right: 5px;
    }

    .install {
      box-shadow: 0 0 6px 1px rgba(59, 130, 246, 0.3),
        0 0 12px 3px rgba(96, 165, 250, 0.2);
    }

    .image-slot {
      display: flex;
      justify-content: center;
      align-items: center;
      width: 100%;
      height: 100%;
      background: var(--el-fill-color-light);
      color: var(--el-text-color-secondary);
      font-size: 30px;
    }

  }

  &.list {
    .item {
      box-sizing: border-box;
      flex-direction: row;
      align-items: center;
      width: 100%;
      padding: 5px 20px;
      border-top: 1px solid #ebeef5;

      .text {
        margin-top: 0;
      }
    }

    .item:nth-child(odd) {
      background-color: #FAFAFA;
    }

    .item:nth-child(even) {
      background-color: #fff;
    }

    .item:hover {
      background-color: #f5f7fa;
    }

    .install {
      box-shadow: 0 0 6px 1px rgba(59, 130, 246, 0.3),
        0 0 12px 3px rgba(96, 165, 250, 0.2);
    }


    .image-slot {
      box-sizing: border-box;
      width: 40px;
      height: 40px;
      text-align: center;
      line-height: 40px;
      border-radius: 50%;
      background-color: #f0f0f0;
      border: 1px solid #f0f0f0;
      color: #999;

    }
  }
}
</style>