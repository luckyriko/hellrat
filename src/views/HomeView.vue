<template>
  <div class="container" @click="displayNoneMenu">
    <!-- 工具栏 -->
    <div class="header">
      <el-popconfirm :title="$t('home.header.addButton.title')" @confirm="openUploadModWindow" @cancel="getModsRecords"
        placement="bottom" :cancel-button-text="$t('home.header.addButton.select')"
        :confirm-button-text="$t('home.header.addButton.upload')" :width="250">
        <template #reference>
          <el-button type="warning" :icon="DocumentAdd" plain>{{ $t('home.header.addButton.text') }}</el-button>
        </template>
      </el-popconfirm>
      <el-button type="info" :icon="Operation" plain @click="toggleView">{{ listShowType == 'grid' ?
        $t('home.header.showType.icon') : $t('home.header.showType.list')
      }}</el-button>
      <el-button type="success" :icon="Sort" plain @click="searchModByOrder">{{ orderAscFlag ?
        $t('home.header.sortType.asc') :
        $t('home.header.sortType.desc')
      }}</el-button>


      <div style="margin-left: 20px;">
        <el-select v-model="environment" value-key="id" :placeholder="$t('home.header.envSelect.placeholder')"
          style="width: 240px" @change="environmentChange">
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
        <el-input v-model="search" :placeholder="$t('home.header.searchInput.placeholder')" clearable
          @keyup.enter.native="searchMod">
          <template #append>
            <el-button @click="searchMod"><i-ep-search /></el-button>
          </template>
        </el-input>
      </div>

      <el-button type="primary" :icon="MagicStick" plain @click="installMods">{{ $t('home.header.apply') }}</el-button>
      <el-button type="danger" :icon="Delete" plain @click="uninstallMods">{{ $t('home.header.clear') }}</el-button>
    </div>
    <!-- 工具栏. -->

    <!-- 列表 -->
    <el-scrollbar class="main-scroll">
      <el-checkbox-group v-model="checkedMods" @change="handleCheckedModsChange" size="large">
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
            <div class="checkbox" v-show="checkboxEditFlag">
              <el-checkbox :value="item.env_mod_id" />
            </div>
          </div>
        </div>
        <div v-show="listShowType != 'grid'" class="content-area list" id="sortableList">
          <div v-for="item in modsData" :key="item.id" class="item">
            <el-checkbox :value="item.env_mod_id" style="margin-right: 12px;" v-show="checkboxEditFlag" />

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
              <div class="text" @click="openSelectOptionsDialog(item.id, item.type)">
                <span>{{ item.name }}</span>
                <el-icon size="20" color="violet" v-if="item.type == 2"><i-ep-edit /></el-icon>
              </div>
              <div>
                <el-button size="small" plain @click="openDir(item)">
                  {{ $t('home.listButton.open') }}
                </el-button>
                <el-button size="small" plain @click="openModInfoDialog(item.id)">
                  {{ $t('home.listButton.detail') }}
                </el-button>
                <el-button size="small" plain @click="handleEdit(item)">
                  {{ $t('home.listButton.edit') }}
                </el-button>
                <el-button type="danger" size="small" plain @click="deleteMod(item)">
                  {{ $t('home.listButton.delete') }}
                </el-button>
              </div>
            </div>
          </div>
        </div>
      </el-checkbox-group>
    </el-scrollbar>
    <!-- 列表. -->

    <!-- 右键功能 -->
    <Menu ref="menuRef" :mentList="mentList" @select-label="selectMenuLabel"></Menu>
    <!-- 右键功能. -->

    <div class="footer">
      <div class="text" style="height: 100%;">
        <div @click="checkboxEditFlag = !checkboxEditFlag" style="margin-right: 18px;" class="text">
          <el-icon>
            <InfoFilled :color="!checkboxEditFlag ? 'orange' : 'red'" />
          </el-icon>
        </div>
        <div v-show="checkboxEditFlag" class="text">
          <el-checkbox v-model="checkAll" :indeterminate="isIndeterminate" @change="handleCheckAllChange"
            style="margin-right: 12px;">
            {{ $t('home.footer.selectAll') }} </el-checkbox>
          <el-button plain type="success" @click="activateAllMods" size="small">{{ $t('home.footer.on') }}</el-button>
          <el-button plain type="warning" @click="disableAllMods" size="small">{{ $t('home.footer.off') }}</el-button>
          <el-button plain type="danger" @click="deleteAllMods" size="small">{{ $t('home.footer.delete') }}</el-button>
        </div>
      </div>
      <div class="text" style="height: 100%;">
        <div @click="donate" class="text">
          <span v-show="donateFlag" style="color: #666;font-size: 0.8rem;">{{ $t('home.footer.donate') }}</span>
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

  <el-dialog v-model="dialogFormVisible" :title="$t('home.editDialog.title')" width="40%" :close-on-click-modal="false">
    <el-form ref="formRef" :model="form" :rules="rules">
      <el-form-item :label="$t('home.editDialog.name')" :label-width="formLabelWidth" prop="name">
        <el-input v-model="form.name" autocomplete="off" />
      </el-form-item>
      <el-form-item :label="$t('home.editDialog.icon')" :label-width="formLabelWidth" prop="icon">
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
      <el-form-item :label="$t('home.editDialog.author')" :label-width="formLabelWidth" prop="author">
        <el-input v-model="form.author" autocomplete="off" />
      </el-form-item>
      <el-form-item :label="$t('home.editDialog.version')" :label-width="formLabelWidth" prop="version">
        <el-input v-model="form.version" autocomplete="off" />
      </el-form-item>
      <el-form-item :label="$t('home.editDialog.link')" :label-width="formLabelWidth" prop="link">
        <el-input v-model="form.link" autocomplete="off" />
      </el-form-item>
      <el-form-item :label="$t('home.editDialog.description')" :label-width="formLabelWidth" prop="desc">
        <el-input v-model="form.desc" autocomplete="off" type="textarea" />
      </el-form-item>
      <el-form-item :label="$t('home.editDialog.imgPath')" :label-width="formLabelWidth" prop="preview"
        style="display: none;">
        <el-input v-model="form.preview" autocomplete="off" disabled />
      </el-form-item>
      <el-form-item :label="$t('home.editDialog.imgLink')" :label-width="formLabelWidth" prop="previewPath"
        style="display: none;">
        <el-input v-model="form.previewPath" autocomplete="off" disabled />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="dialogFormVisible = false">{{ $t('home.editDialog.cancel') }}</el-button>
        <el-button @click="resetForm(formRef)">{{ $t('home.editDialog.reset') }}</el-button>
        <el-button type="primary" :loading="loadingFlag" @click="onSubmit(formRef)">
          {{ $t('home.editDialog.confirm') }} </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<script setup>
import { h, ref, reactive, onMounted, toRaw, computed, nextTick } from 'vue';
import { open as openShell } from '@tauri-apps/plugin-shell';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { join } from '@tauri-apps/api/path';
import SelectOptions from './components/SelectOptions.vue';
import AddEnvironments from './components/AddEnvironments.vue';
import SelectMods from './components/SelectMods.vue';
import ShowModInfo from './components/ShowModInfo.vue';
import Menu from './components/Menu.vue';
import Sortable from 'sortablejs';
import { DocumentAdd, Delete, EditPen, Operation, Sort, MagicStick, ColdDrink, InfoFilled } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-shell';
import { listen } from '@tauri-apps/api/event';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const checkboxEditFlag = ref(false);
const checkAll = ref(false);
const isIndeterminate = ref(false);
const checkedMods = ref([]);

const handleCheckAllChange = (flag) => {
  // console.log("handleCheckAllChange: ", flag);
  checkedMods.value = [];
  isIndeterminate.value = false;

  if (flag) {
    modsData.value.forEach(element => {
      checkedMods.value.push(element.env_mod_id);
    });
  }

}
const handleCheckedModsChange = (value) => {
  // console.log("handleCheckedModsChange: ", value);
  const checkedCount = value.length
  checkAll.value = checkedCount === modsData.value.length
  isIndeterminate.value = checkedCount > 0 && checkedCount < modsData.value.length
}


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
    console.log(environment.value);
    invoke("update_environment_activate", { id: item.id }).then(async () => {
      await getEnvironmentList();
      await getModsList();
    });
  } catch (error) {
    ElMessage.error(t('home.header.envSelect.changeError'))

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
    ElMessage.error(t('home.changeModActivateError'))

  }
}

const activateAllMods = async () => {
  if (checkedMods.value.length == 0) {
    ElMessage.error(t('home.activateAllMods.tip'));
    return
  }

  const loading = ElLoading.service({
    lock: true,
    text: t('home.activateAllMods.loadingText'),
    background: 'rgba(0, 0, 0, 0.7)',
  })
  try {
    await Promise.allSettled(checkedMods.value.map(async env_mod_id => {
      await invoke("update_environment_mod_activate", { id: env_mod_id, activate: 1 });
    }));

  } catch (error) {
    ElMessage.error(t('home.activateAllMods.error') + String(error))

  } finally {
    loading.close();
    await getModsList();

  }
}

const disableAllMods = async () => {
  if (checkedMods.value.length == 0) {
    ElMessage.error(t('home.disableAllMods.tip'));
    return
  }

  const loading = ElLoading.service({
    lock: true,
    text: t('home.disableAllMods.loadingText'),
    background: 'rgba(0, 0, 0, 0.7)',
  })
  try {
    await Promise.allSettled(checkedMods.value.map(async env_mod_id => {
      await invoke("update_environment_mod_activate", { id: env_mod_id, activate: 0 });
    }));
  } catch (error) {
    ElMessage.error(t('home.disableAllMods.error') + String(error))

  } finally {
    loading.close();
    await getModsList();

  }
}

const deleteAllMods = async () => {
  if (!gameMod.mods_store_path) {
    ElMessage.error(t('home.deleteAllMods.modsStorePathError'));
    return
  }

  if (!environment.value.id) {
    ElMessage.error(t('home.deleteAllMods.envIdError'));
    return
  }

  if (checkedMods.value.length == 0) {
    ElMessage.error(t('home.deleteAllMods.selectError'));
    return
  }

  const delete_file_flag = ref(false);
  ElMessageBox({
    title: t('home.deleteAllMods.messageBox.title'),
    message: () =>
      h('div', null, [
        h('span', null, t('home.deleteAllMods.messageBox.desc')),
        h(ElSwitch, {
          modelValue: delete_file_flag.value,
          'onUpdate:modelValue': (val) => {
            delete_file_flag.value = val
          },
        }),
      ]),
    showCancelButton: true,
    confirmButtonText: t('home.deleteAllMods.messageBox.confirm'),
    cancelButtonText: t('home.deleteAllMods.messageBox.cancel'),
  }).then(async (action) => {
    if (action == 'confirm') {
      const loading = ElLoading.service({
        lock: true,
        text: t('home.deleteAllMods.messageBox.loadingText'),
        background: 'rgba(0, 0, 0, 0.7)',
      })

      try {
        const results = await Promise.allSettled(checkedMods.value.map(async env_mod_id => {
          let mod = modsData.value.find(x => x.env_mod_id == env_mod_id);
          if (mod) {
            await invoke("delete_one_mod", {
              record_id: mod.id,
              env_id: Number(environment.value.id),
              delete_file_flag: delete_file_flag.value ? 1 : 0,
              mod_dir_name: mod.path
            });
          }
        }));

        const failed = results
          .filter(r => r.status === "rejected")
          .map(r => r.reason);

        console.log("失败的结果:", failed);

        if (failed.length == 0) {
          ElMessage({
            message: t('home.deleteAllMods.messageBox.success'),
            type: 'success'
          })
        } else {
          const uniqueFailed = [...new Set(failed)];
          ElMessage.error(t('home.deleteAllMods.messageBox.someFail', { count: failed.length }) + uniqueFailed.join(''));

        }

      } catch (error) {
        console.error("Error delete mod:", error);
        ElMessage.error(t('home.deleteAllMods.messageBox.fail') + String(error))

      } finally {
        loading.close();
        await getModsList();

      }
    }
  }).catch(() => {
    ElMessage({
      type: 'info',
      message: t('home.deleteAllMods.messageBox.cancelTip'),
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
    console.error("排序值更新失败", error);
    ElMessage.error(t('home.sort.error'))

  }
}

const mentList = [
  {
    label: t('home.rightClick.open'),
    operation: 'openDir'
  },
  {
    label: t('home.rightClick.edit'),
    operation: 'handleEdit'
  },
  {
    label: t('home.rightClick.delete'),
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
    title: t('home.editDialog.imgTip'),
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
    { required: true, min: 1, max: 255, message: t('home.editDialog.nameTip'), trigger: ['blur', 'change'] }
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
      title: t('home.uploadWindow.title'),
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
    text: t('home.installMods.loadingText'),
    background: 'rgba(0, 0, 0, 0.7)',
  })

  try {
    await invoke("install_mods", { env_id: Number(environment.value.id) });
    ElMessage({
      message: t('home.installMods.success'),
      type: 'success',
    })
  } catch (error) {
    console.error("安装失败", error);
    ElMessage.error(error || t('home.installMods.fail'))

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
          message: t('home.editDialog.success'),
          type: 'success',
        })

      } catch (error) {
        console.error('Failed to submit:', error);
        ElMessage.error(String(error) || t('home.editDialog.fail'))

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
    ElMessage.error(t('home.openDir.modsStorePathError'))
    return;
  }
  try {
    let dir_path = await join(gameMod.mods_store_path, row.path);
    await invoke('open_folder', { path: dir_path });
    // console.log('Folder opened successfully!');
  } catch (error) {
    console.error('打开Mod目录失败:', error);
    ElMessage.error(error || t('home.openDir.fail'))

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
    ElMessage.error(error || t('home.environment.fail'))

  }
}


// 一键卸载
const uninstallMods = async () => {
  if (!gameMod.game_data_path) {
    ElMessage.error(t('home.uninstallMods.gameDataPathError'));
    return;
  }

  const delete_all_flag = ref(false);

  ElMessageBox({
    title: t('home.uninstallMods.messageBox.title'),
    message: () =>
      h('div', null, [
        h('span', null, t('home.uninstallMods.messageBox.desc')),
        h(ElSwitch, {
          modelValue: delete_all_flag.value,
          'onUpdate:modelValue': (val) => {
            delete_all_flag.value = val
          },
        }),
      ]),
    showCancelButton: true,
    confirmButtonText: t('home.uninstallMods.messageBox.confirm'),
    cancelButtonText: t('home.uninstallMods.messageBox.cancel'),
  }).then(async (action) => {
    if (action == 'confirm') {
      const loading = ElLoading.service({
        lock: true,
        text: t('home.uninstallMods.messageBox.loadingText'),
        background: 'rgba(0, 0, 0, 0.7)',
      })

      try {
        await invoke("uninstall_mods", {
          all_mod_flag: delete_all_flag.value ? 1 : 0
        });
        ElMessage({
          message: t('home.uninstallMods.messageBox.success'),
          type: 'success'
        })

      } catch (error) {
        console.error("Error delete mod:", error);
        ElMessage.error(t('home.uninstallMods.messageBox.fail') + String(error))

      } finally {
        loading.close();
        await getModsList();

      }
    }
  }).catch(() => {
    ElMessage({
      type: 'info',
      message: t('home.uninstallMods.messageBox.cancelTip'),
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
    ElMessage.error(t('home.deleteMod.modsStorePathError'));
    return
  }

  if (!environment.value.id) {
    ElMessage.error(t('home.deleteMod.envIdError'));
    return
  }

  const delete_file_flag = ref(false);
  ElMessageBox({
    title: t('home.deleteMod.messageBox.title'),
    message: () =>
      h('div', null, [
        h('span', null, t('home.deleteMod.messageBox.desc')),
        h(ElSwitch, {
          modelValue: delete_file_flag.value,
          'onUpdate:modelValue': (val) => {
            delete_file_flag.value = val
          },
        }),
      ]),
    showCancelButton: true,
    confirmButtonText: t('home.deleteMod.messageBox.confirm'),
    cancelButtonText: t('home.deleteMod.messageBox.cancel'),
  }).then(async (action) => {
    if (action == 'confirm') {
      const loading = ElLoading.service({
        lock: true,
        text: t('home.deleteMod.messageBox.loadingText'),
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
          message: t('home.deleteMod.success'),
          type: 'success'
        })
        await getModsList();

      } catch (error) {
        console.error("Error delete mod:", error);
        ElMessage.error(t('home.deleteMod.fail') + String(error))

      } finally {
        loading.close();

      }
    }
  }).catch(() => {
    ElMessage({
      type: 'info',
      message: t('home.deleteMod.cancelTip'),
    })
  })

}

const orderAscFlag = ref(false);

// 获取mod存档
async function getModsList(name = '') {
  if (!name) {
    search.value = '';
  }
  const loading = ElLoading.service({
    lock: true,
    text: t('home.getModsList.loadingText'),
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
    ElMessage.error(t('home.getModsList.fail'))

  } finally {
    checkedMods.value = [];
    isIndeterminate.value = false;
    checkAll.value = false;
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
      ElMessage.error(t('home.getModsRecords.zeroTip'))

    }
  } catch (error) {
    console.error("Error get records:", error);
    ElMessage.error(t('home.getModsRecords.fail'))

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
  box-sizing: border-box;
}

.header {
  height: 50px;
  border-top: 1px solid #ddd;
  justify-content: space-between;
}

.footer {
  height: 40px;
  border-top: 1px solid #ddd;
  justify-content: space-between;
}

.main-scroll {
  box-sizing: border-box;
}

.text {
  display: flex;
  flex-direction: row;
  align-items: center;
}

:deep(.el-checkbox-group) {
  box-sizing: border-box;
  font-size: 1rem;
  line-height: normal;
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

.sortable-ghost {
  color: white;
  background-color: skyblue !important;
}


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

    .checkbox {
      position: absolute;
      top: 0;
      left: 8px;
      // 防止误点
      padding-right: 18px;
      padding-bottom: 18px;
      // background-color: red;
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