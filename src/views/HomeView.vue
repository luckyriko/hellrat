<template>
  <div class="container">
    <el-row>
      <el-col :span="24">
        <div class="title">Mod列表</div>
      </el-col>
    </el-row>

    <el-row :gutter="20">
      <el-col :span="3">
        <el-segmented v-model="current" :options="options" size="default" @change="searchMod"/>
      </el-col>
      <el-col :span="9">
        <el-input v-model="search" placeholder="请输入名称" clearable>
          <template #append>
            <el-button @click="searchMod"><i-ep-search /></el-button>
          </template>
        </el-input>
      </el-col>

      <el-col :span="3" :offset="6">
        <el-button @click="uninstallAllMods" type="danger" plain>一键卸载</el-button>
      </el-col>
      <el-col :span="3">
        <el-button @click="deploy" type="primary" plain>应用安装</el-button>
      </el-col>
    </el-row>

    <el-table ref="tableRef" :default-sort="{ prop: 'id', order: 'descending' }" :data="modsData" max-height="600"
      style="width: 100%" row-key="id" stripe @expand-change="getModDetail">
      <el-table-column type="expand">
        <template #default="props">
          <div style="margin-left: 20px;" v-if="props.row?.files && props.row.files.length > 0">
            <p v-for="(item, index) in props.row.files" :key="index">{{ item }}</p>
          </div>
          <div style="margin-left: 20px;" v-else>未安装</div>
        </template>
      </el-table-column>
      <el-table-column prop="id" label="ID" sortable min-width="60" align="center" />
      <el-table-column prop="name" label="名称" min-width="120">
        <template #default="scope">
          <div style="display: flex; flex-direction: row; align-items: center;">
            <div v-if="scope.row.link ? true : false">
              <el-link :underline="true" type="primary" @click="openBrowser(scope.row.link)">{{ scope.row.name
                }}</el-link>
            </div>
            <div v-else>
              {{ scope.row.name }}
            </div>

            <div v-if="scope.row.preview" style="display: flex; flex-direction: row; align-items: center;">
              <el-popover :width="500" placement="right"
                popper-style="box-shadow: rgb(14 18 22 / 35%) 0px 10px 38px -10px, rgb(14 18 22 / 20%) 0px 10px 20px -15px; padding: 20px;">
                <template #reference>
                  <el-icon size="20" color="purple"><i-ep-picture-rounded /></el-icon>
                </template>
                <template #default>
                  <el-image style="width: 450px" :src="scope.row.preview" fit="fill" />
                </template>
              </el-popover>
            </div>
          </div>
        </template>
      </el-table-column>
      <!-- <el-table-column prop="mod_type" label="类型" :formatter="type" align="center" /> -->
      <el-table-column prop="activate" label="安装" :filters="[
        { text: '已安装', value: true },
        { text: '未安装', value: false },
      ]" :filter-method="filterHandler" filter-placement="bottom-end" align="center">
        <template #default="scope">
          <!-- <el-tag :type="scope.row.activate ? 'success' : 'danger'" disable-transitions>{{ scope.row.activate ?
            '已安装' : '未安装' }}</el-tag> -->
          <el-switch v-model="scope.row.activate" inline-prompt @change="activateChange(scope.row)"
            style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949" active-text="是" inactive-text="否" />
        </template>
      </el-table-column>
      <el-table-column prop="author" label="作者" align="center" />
      <!-- <el-table-column prop="link" label="链接" /> -->
      <el-table-column property="desc" label="详情" width="240" show-overflow-tooltip align="center" />
      <!-- <el-table-column prop="preview" label="预览图" /> -->
      <el-table-column fixed="right" label="操作" min-width="180" align="center">
        <template #default="scope">
          <el-button size="small" @click="openDir(scope.row.name)">
            打开
          </el-button>
          <el-button size="small" @click="handleEdit(scope.$index, scope.row)">
            编辑
          </el-button>
          <el-button type="danger" size="small" @click.prevent="deleteMod(scope.$index, scope.row)">
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>
    <el-button class="mt-4" style="width: 100%" @click="onAddItem">
      Add Item
    </el-button>
  </div>
</template>
<script setup>
import { ref, reactive, onMounted, toRaw, computed } from 'vue'
import { open } from '@tauri-apps/plugin-shell';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { exists, mkdir, create, readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';

const current = ref('模型');
const options = ['模型', '音频'];

// 搜索
const search = ref('')

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
  } catch (error) {

  }

  await getModsList();
});

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
      console.log(mod_activate_data);

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
  await open(url);

}

// 打开mod对应的存档目录
async function openDir(name) {
  if (!modsDir.value) {
    ElMessage.error('游戏存档目录未设置')
    return;
  }
  try {
    let dir_path = await join(modsDir.value, name);
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
        await invoke("uninstall_mods_all", { data_dir: gameDataDir.value });
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
const deleteMod = async (index, row) => {
  if (Number(row.activate) === 1) {
    ElMessage.error('请先卸载mod后再进行删除');
    return
  }
  if (!modsDir.value) {
    ElMessage.error('未获取到mod存档目录');
    return
  }
  const loading = ElLoading.service({
    lock: true,
    text: '删除中...',
    background: 'rgba(0, 0, 0, 0.7)',
  })

  try {
    let dir_path = await join(modsDir.value, row.name);
    await invoke("remove_dir_all", { dir_path, record_id: row.id });
    modsData.value.splice(index, 1);
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
  return row.mod_type === 'model' ? '模型' : '音频';
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
      element.preview = await previewImg(element);
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


const handleEdit = (index, row) => {
  console.log(index, row)
}

// 测试添加
const onAddItem = () => {
  const maxId = modsData.value.reduce((max, current) => {
    return current.id > max ? current.id : max;
  }, -Infinity);
  modsData.value.push({
    id: Number(maxId) + 1,
    name: 'Tom',
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

    return assetUrl
  } else {
    return '';
  }
};


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