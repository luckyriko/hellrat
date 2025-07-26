<template>
  <div class="container" @click="displayNoneMenu">
    <div class="header">
      <el-button @click="toggleView" icon="Switch" circle>切换</el-button>

      <!-- 标题 -->
      <!-- <el-row>
        <el-col :span="24">
          <div class="title">列表</div>
        </el-col>
      </el-row> -->
      <!-- 标题. -->

      <!-- 工具栏 -->
      <!-- <el-row :gutter="20">
        <el-col :span="4">
          <el-segmented v-model="current" :options="options" size="default" @change="searchMod" />
        </el-col>
        <el-col :span="8">
          <el-input v-model="search" placeholder="请输入名称" clearable @keyup.enter.native="searchMod">
            <template #append>
              <el-button @click="searchMod"><i-ep-search /></el-button>
            </template>
</el-input>
</el-col>

<el-col :span="3">
  <el-button @click="activeAllMods" type="success" plain>全部启用</el-button>
</el-col>
<el-col :span="3">
  <el-button @click="deploy" type="primary" plain>应用安装</el-button>
</el-col>
<el-col :span="3">
  <el-button @click="toSort" color="orange" plain>编辑排序</el-button>
</el-col>
<el-col :span="3" :offset="0">
  <el-button @click="uninstallAllMods" type="danger" plain>一键卸载</el-button>
</el-col>
</el-row> -->
      <!-- <el-button class="mt-4" style="width: 100%" @click="onAddItem">
      测试新增
    </el-button> -->
      <!-- 工具栏. -->
    </div>

    <!-- 表格 -->
    <!-- <el-scrollbar class="content">
      <div class="card-grid" id="sortableList">
        <div class="card" v-for="item in tableData" >
          <img src="https://luckyriko.com/assets/images/cover3.jpg" alt="示例图片">
          <div class="card-text">
            <h3>{{item.name}}</h3>
            <p>卡片描述信息</p>
          </div>
        </div>

      </div>
    </el-scrollbar> -->


    <el-scrollbar class="main-scroll">
      <div :class="['content-area', viewMode]" id="sortableList">
        <div v-for="item in tableData" :key="item.id" class="item">
          <img src="https://luckyriko.com/assets/images/cover3.jpg" alt="图" />
          <div class="text">{{ item.name }}</div>
        </div>
      </div>
    </el-scrollbar>

    <!-- 表格. -->

    <!-- 右键功能 -->
    <!-- <Menu ref="menuRef" :mentList="mentList" @select-label="selectMenuLabel"></Menu> -->
    <!-- 右键功能. -->

    <div class="footer">底部固定区域</div>

  </div>

  <SortableDiglog v-model="dialogVisible" @close="closeDialog"></SortableDiglog>

  <el-dialog v-model="dialogFormVisible" title="修改Mod信息" width="500" :close-on-click-modal="false">
    <el-form ref="formRef" :model="form" :rules="rules">
      <el-form-item label="名称" :label-width="formLabelWidth" prop="memo">
        <el-input v-model="form.memo" autocomplete="off" />
      </el-form-item>
      <el-form-item label="类型" :label-width="formLabelWidth" prop="mod_type">
        <!-- <el-select v-model="form.region" placeholder="请选择Mod类型">
          <el-option label="模型" value="model" />
          <el-option label="音频" value="voice" />
        </el-select> -->
        <el-radio-group v-model="form.mod_type">
          <el-radio value="model">模型</el-radio>
          <el-radio value="voice">音频/杂项</el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item label="预览图" :label-width="formLabelWidth" prop="preview">
        <div v-if="form.previewPath" class="image" @click="selectPreViewImg">
          <el-image style="height: 150px" :src="form.previewPath" fit="fill" />
          <el-icon class="close-icon" size="30" color="red"
            @click.stop="removeImg"><i-ep-circle-close-filled /></el-icon>
        </div>
        <div v-else class="image" @click="selectPreViewImg">
          <el-icon size="20" color="#666"><i-ep-plus /></el-icon>
        </div>
        <el-input v-model.trim="form.preview" type="hidden" />
      </el-form-item>
      <el-form-item label="作者" :label-width="formLabelWidth" prop="author">
        <el-input v-model="form.author" autocomplete="off" />
      </el-form-item>
      <el-form-item label="链接" :label-width="formLabelWidth" prop="link">
        <el-input v-model="form.link" autocomplete="off" />
      </el-form-item>
      <el-form-item label="详情" :label-width="formLabelWidth" prop="desc">
        <el-input v-model="form.desc" autocomplete="off" type="textarea" />
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
import { ref, reactive, onMounted, toRaw, computed } from 'vue'
import { open as openShell } from '@tauri-apps/plugin-shell';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { exists, mkdir, create, readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';
import SortableDiglog from './components/SortableDiglog.vue';
import Menu from './components/Menu.vue';
import Sortable from 'sortablejs'

const viewMode = ref('grid') // grid or list

const toggleView = () => {
  viewMode.value = viewMode.value === 'grid' ? 'list' : 'grid'
}

// 排序数据
const tableData = [
  {
    date: '2016-05-03',
    name: 'Tom',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-02',
    name: 'Cilly',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-04',
    name: 'Linda',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-01',
    name: 'John',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-03',
    name: 'Tom',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-02',
    name: 'Cilly',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-04',
    name: 'Linda',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-01',
    name: 'John',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-03',
    name: 'Tom',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-02',
    name: 'Cilly',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-04',
    name: 'Linda',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-01',
    name: 'John',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-03',
    name: 'Tom',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-02',
    name: 'Cilly',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-04',
    name: 'Linda',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-01',
    name: 'John',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-03',
    name: 'Tom',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-02',
    name: 'Cilly',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-04',
    name: 'Linda',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-01',
    name: 'John',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-03',
    name: 'Tom',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-02',
    name: 'Cilly',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-04',
    name: 'Linda',
    address: 'No. 189, Grove St, Los Angeles',
  },
  {
    date: '2016-05-01',
    name: 'John',
    address: 'No. 189, Grove St, Los Angeles',
  },
]

function setSort() {
  const el = document.getElementById('sortableList');
  new Sortable(el, {
    sort: true,
    animation: 150,
    swapThreshold: 0.5,

    // handle: '.handle-drag',
    ghostClass: 'sortable-ghost',
    // easing: 'cubic-bezier(1, 0, 0, 1)',
    onStart: (e) => {
      console.log('onstart:', e);
    },
    onEnd: (e) => {
      // const targetRow = tableData.splice(e.oldIndex, 1)[0]
      // tableData.splice(e.newIndex, 0, targetRow)
      console.log('onEnd:', e)

      submitSortResult()

    },
  })
}

const submitSortResult = () => {
  console.log('submitSortResult')
}

const current = ref('模型');
const options = ['模型', '音频/杂项'];
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
const handleContextMenu = (row, column, event) => {
  //阻止浏览器默认事件
  event.preventDefault();

  // 通过获取到的组件实例，设置菜单的显示和位置
  menuRef.value.show = true;
  menuRef.value.setPosition(event.clientX, event.clientY);
  currentMenuData.value = row
}

const selectMenuLabel = (menuItem) => {
  console.log(currentMenuData.value, menuItem.operation);
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

// 排序 
// 定义一个响应式变量，用于控制对话框的显示与隐藏
const dialogVisible = ref(false);

// 打开对话框的函数，将 dialogVisible 设置为 true
const toSort = () => {
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
  memo: '',
  directory: '',
  author: '',
  link: '',
  mod_type: '',
  desc: '',
  preview: '',
  previewPath: ''
});
const loadingFlag = ref(false);

const selectPreViewImg = async () => {
  const selectedPath = await openDialog({
    multiple: false,
    directory: false,
    title: "请选择预览图",
  });


  if (selectedPath) {
    form.preview = selectedPath;
    form.previewPath = convertFileSrc(selectedPath);
  }

};
const removeImg = () => {
  form.preview = '';
  form.previewPath = '';
};

const rules = reactive({
  memo: [
    { required: true, min: 1, max: 255, message: '最少1个字符，最长255个字符', trigger: ['blur', 'change'] }
  ],
  mod_type: [
    { required: true, message: '请选择mod类型', trigger: ['blur', 'change'] },
  ]
})

const handleEdit = (row) => {
  if (row) {
    Object.assign(form, row);
    dialogFormVisible.value = true;
  }
}


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
        let mod = modsData.value.find(x => x.id == form.id);
        let up_img_flag = true;
        if (mod.preview == form.preview) {
          console.log('图片无需更新');
          up_img_flag = false;
        }

        await invoke('up_mod_info', { mod_info: form, record_dir: modsDir.value, up_img_flag });
        console.log('up_mod_info successfully!');
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

// 挂载时获取设置参数里的各种目录
onMounted(async () => {
  try {
    const contents = await readTextFile('config.json', {
      baseDir: BaseDirectory.AppConfig,
    });

    if (contents) {
      const config = JSON.parse(contents);
      modsDir.value = config.modsDir || '';
      gameDataDir.value = config.gameDataDir || '';

      if (!gameDataDir.value || !modsDir.value) {
        ElMessage.error('请先去设置里添加游戏data目录和mod存档目录')
      }
    } else {
      ElMessage.error('请先去设置里添加游戏data目录和mod存档目录')
    }

    setSort();

  } catch (error) {

  }

  await getModsList();
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

async function deploy() {
  if (!modsDir.value || !gameDataDir.value) {
    ElMessage.error('请先去设置里添加游戏data目录和mod存档目录');
    return;
  }

  ElMessageBox.confirm(
    '此操作将所有mod卸载后，将根据列表中的安装状态，重新进行mod安装，确定继续吗？',
    '提示',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  )
    .then(async () => {
      const loading = ElLoading.service({
        lock: true,
        text: '部署中...',
        background: 'rgba(0, 0, 0, 0.7)',
      })


      let mod_activate_data = [];
      modsData.value.forEach(element => {
        mod_activate_data.push({
          id: element.id,
          activate: element.activate
        })
      });
      // console.log(mod_activate_data);

      try {
        await invoke('deploy_mods', {
          record_dir: modsDir.value,
          data_dir: gameDataDir.value,
          mode_type: current.value == '模型' ? 'model' : 'voice',
          mod_activate_data
        });

        await getModsList();

        ElMessage({
          message: '部署成功',
          type: 'success',
        })
      } catch (error) {
        console.error('deploy mods err:', error);
        ElMessage.error(error || '部署失败')

      } finally {
        loading.close();

      }
    })
    .catch(() => {
      ElMessage({
        type: 'info',
        message: '已取消',
      })
    })
}

async function searchMod() {
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
  if (!modsDir.value) {
    ElMessage.error('游戏存档目录未设置')
    return;
  }
  try {
    let dir_path = await join(modsDir.value, row.name);
    await invoke('open_folder', { path: dir_path });
    // console.log('Folder opened successfully!');
  } catch (error) {
    console.error('Failed to open folder:', error);
    ElMessage.error(error || 'Oops, this is a error message.')

  }
}

// 一键卸载
const uninstallAllMods = async () => {
  if (!gameDataDir.value) {
    ElMessage.error('游戏data目录未设置');
    return;
  }

  ElMessageBox.confirm(
    '此操作将删除游戏data目录下，通过本软件安装的所有mod，确定继续吗？',
    '提示',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  )
    .then(async () => {
      const loading = ElLoading.service({
        lock: true,
        text: '卸载中...',
        background: 'rgba(0, 0, 0, 0.7)',
      })

      try {
        let mod_type = current.value == '模型' ? 'model' : 'voice';
        await invoke("uninstall_mods_all", { data_dir: gameDataDir.value, mod_type });
        await getModsList();

        ElMessage({
          message: '卸载成功',
          type: 'success',
        })
      } catch (error) {
        console.error("Error uninstall all mods:", error);
        ElMessage.error('卸载失败：' + String(error))

      } finally {
        loading.close();

      }
    })
    .catch(() => {
      ElMessage({
        type: 'info',
        message: '已取消',
      })
    })
}

// 删除mod存档
const deleteMod = async (row) => {
  if (Number(row.activate) === 1) {
    ElMessage.error('该mod正在使用，请先卸载后再删除');
    return
  }
  if (!modsDir.value) {
    ElMessage.error('未获取到mod存档目录');
    return
  }

  ElMessageBox.confirm(
    '此操作将永久删除该Mod存档，确定继续吗？',
    '提示',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  )
    .then(async () => {

      const loading = ElLoading.service({
        lock: true,
        text: '删除中...',
        background: 'rgba(0, 0, 0, 0.7)',
      })

      try {
        let dir_path = await join(modsDir.value, row.name);
        await invoke("remove_dir_all", { dir_path, record_id: row.id });
        getModsList();
        ElMessage({
          message: '删除成功',
          type: 'success'
        })
      } catch (error) {
        console.error("Error delete mod:", error);
        ElMessage.error('删除失败：' + String(error))

      } finally {
        loading.close();

      }
    })
    .catch(() => {
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
  console.log(value, row.activate);
  return row.activate === value
}

const type = (row, column) => {
  return row.mod_type === 'model' ? '模型' : '音频/杂项';
}

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
    let modeType = current.value == '模型' ? 'model' : 'voice';
    let mods = await invoke("get_mod_records", { search: name, modeType });
    mods.forEach(async element => {
      let { preview, assetUrl } = await previewImg(element);
      element.preview = preview;
      element.previewPath = assetUrl;

    });

    modsData.value = mods;
    ElMessage({
      message: '获取列表成功',
      type: 'success',
      duration: 1000
    })
    // console.log("get mod records:", modsData.value);
  } catch (error) {
    console.error("Error get records:", error);
    ElMessage.error('获取失败')

  } finally {
    loading.close();
    getStatistics();

  }
}

// 获取mod被安装在data目录的名称
async function getModDetail(row) {
  try {
    let files = await invoke("get_mod_install_files", { recordId: row.id });
    row.files = files || [];
    // console.log("get mod install files:", files);
  } catch (error) {
    console.error("Error get mod install files:", error);
    ElMessage.error('获取失败')

  } finally {

  }

}

// 测试添加
const onAddItem = () => {
  const maxId = modsData.value.reduce((max, current) => {
    return current.id > max ? current.id : max;
  }, -Infinity);
  modsData.value.push({
    id: Number(maxId) + 1,
    memo: 'Tom',
    mod_type: 'California',
    activate: 'Los Angeles',
    desc: 'No. 189, Grove St, Los Angeles',
    author: 'CA 90036',
  })
}

// 预览图片
const previewImg = async (row) => {
  if (modsDir.value && row.name && row.preview) {

    let imgPath = await join(modsDir.value, row.name, row.preview);

    const assetUrl = convertFileSrc(imgPath);

    // console.log('---imgPath---', assetUrl);

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
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  box-sizing: border-box;
  padding: 20px 20px 0 20px;
  position: relative;
}


.header,
.footer {
  height: 60px;
  background: #f5f5f5;
  border-bottom: 1px solid #ddd;
}

.footer {
  border-top: 1px solid #ddd;

  padding: 4px;
  // background-color: red;
}


.content {
  flex: 1; // ✅ 自动撑满剩余空间
  overflow: auto; // ✅ 激活 el-scrollbar 内部滚动
  background: #fff;
  padding: 16px;
}

.title {
  font-size: 20px;
  font-weight: bold;
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

.sortable-ghost {
  color: white;
  background-color: skyblue !important;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: 20px;
  padding: 20px;
  // height: 100%;
  // background-color: red;

  .card {
    background-color: #fff;
    border: 1px solid #eee;
    border-radius: 10px;
    overflow: hidden;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
    transition: transform 0.2s ease;
    display: flex;
    flex-direction: column;

    // &:hover {
    //   transform: translateY(-4px);
    // }

    img {
      width: 100%;
      height: 160px;
      object-fit: cover;
    }

    .card-text {
      padding: 12px;
      font-size: 16px;
      text-align: center;

      h3 {
        margin: 0;
        font-size: 18px;
      }

      p {
        font-size: 14px;
        color: #666;
        margin-top: 6px;
      }
    }
  }
}

.content-area {
  display: grid;
  gap: 16px;
  padding: 16px;

  .item {
    background: #f0f0f0;
    border-radius: 6px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;

    img {
      width: 100%;
      max-width: 120px;
      object-fit: cover;
    }

    .text {
      margin-top: 8px;
    }
  }

  &.grid {
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  }

  &.list {
    grid-template-columns: 1fr;

    .item {
      flex-direction: row;
      align-items: center;

      img {
        width: 120px;
        margin-right: 16px;
      }

      .text {
        margin-top: 0;
      }
    }
  }
}
</style>