<template>
  <div>
    <video controls>
      <source :src="videoPath" type="video/mp4">
    </video>
    <el-button @click="selectFile">选择文件</el-button>
    <el-button @click="bilibiliUpload">上传</el-button>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { ref } from 'vue';

//视频路径
const videoPath = ref("");

const selectFile = async () => {
  const file = await open({
    multiple: false,
    directory: false,
  });
  if (file) {
    videoPath.value = file;
  }
};

const bilibiliUpload = async () => {
  await invoke("bilibili_upload", { ptah: videoPath.value });
};
</script>

<style scoped></style>
