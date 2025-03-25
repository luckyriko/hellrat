<template>
  <el-scrollbar max-height="100vh">
    <div class="container">
      <el-row>
        <el-col :span="24">
          <div class="title">帮助</div>
        </el-col>
      </el-row>

      <el-row>
        <el-col :span="24" class="row">
          <div style="color: blue;">1、如何安装一个mod?</div>
          <div>答：先去设置里填写游戏data路径和Mods存档路径；去安装页选取一个mod路径，将安装置为打开状态，可以修改名称，最后点击安装即可；不打开安装状态则为存档。安装/存档完成后将会显示在列表页。</div>
        </el-col>
        <el-col :span="24" class="row">
          <div style="color: blue;">2、是否支持多个mod同时安装?</div>
          <div>答：不支持多个mod同时安装，只支持单个mod目录添加。</div>
        </el-col>
        <el-col :span="24" class="row">
          <div style="color: blue;">3、安装页，模型、音频/杂项，这两个类型有什么区别?</div>
          <div>
            答：软件会自动帮你识别；模型文件以“9ba626afa44a3aa3.patch_xxx”开头的将会被安装，其它文件不会被安装；音频/杂项则反之，前面不以“9ba626afa44a3aa3”为开头，中间部分必须是“.patch_xxx”（xxx必须为纯数字，如果不是请自行修改）的才会被安装。模型类型：如果目录里没有以“9ba626afa44a3aa3.patch_xxx”开头的文件，则没有文件会被安装；音频/杂项类型：不会安装以“9ba626afa44a3aa3”开头的文件，会安装“yyyy.patch_数字”格式的文件。两者图片都不会被安装。具体安装了哪些文件可以点击列表页某一项的最前面的箭头查看。
          </div>
        </el-col>
        <el-col :span="24" class="row">
          <div style="color: blue;">4、安装页，我没有把安装置为打开状态，这将会发生什么?</div>
          <div>答：这将会把mod目录复制到设置页中的mod存档路径，并记录额外信息，以方便后续安装，存档后可以在列表页进行查看、编辑、安装。打开安装状态会自动存档。</div>
        </el-col>
        <el-col :span="24" class="row">
          <div style="color: blue;">5、列表页，如何安装或卸载列表里被存档的mod?</div>
          <div>答：想要安装的mod把安装列置为“是”，想要卸载的mods把安装列置为“否”，然后点击应用安装。点击应用安装时才会真正安装这些mod到游戏data目录。</div>
        </el-col>
        <el-col :span="24" class="row">
          <div style="color: blue;">6、列表页，点击全部启用/改变列表安装栏状态会立即安装吗?</div>
          <div>答：并不会立即安装，点击应用安装才会真正安装这些mods到游戏data目录。点击搜索可以重置你修改的安装状态到原本状态，即刷新。</div>
        </el-col>
        <el-col :span="24" class="row">
          <div style="color: blue;">7、列表页，点击一键卸载会发生什么?</div>
          <div>答：会删除通过本软件安装到游戏data目录里的所有文件，非本软件安装的文件不会被删除。这意味着你无需担心之前没有使用此软件安装的mods文件会被删除。</div>
        </el-col>
        <el-col :span="24" class="row">
          <div style="color: blue;">8、关于安装顺序?</div>
          <div>答：ID越小的会先被安装，ID越小的也意味着最先被添加的。可以通过删除后再添加来改变安装顺序。</div>
        </el-col>
        <el-col :span="24" class="row">
          <div style="color: blue;">9、列表页，关于删除?</div>
          <div>答：删除需要mod已经被卸载了，可以通过把安装列置为“否”，然后点击应用安装，来卸载mod。删除将清空所有信息并永久删除该mod所对应存档目录下的所有文件。删除前请自行备份以免数据丢失。</div>
        </el-col>
      </el-row>
    </div>
  </el-scrollbar>
</template>

<script setup>


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

.row {
  margin-bottom: 20px;
  /* 设置每一行与下一行的间隔 */
}

div {
  font-size: 14px;
}
</style>
