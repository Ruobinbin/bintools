<template>
    <div>
        <h1>bilibili</h1>
        <el-input v-model="bilibiliRoomId" placeholder="请输入房间号" />
        <el-input v-model="bilibiliCookie" placeholder="请输入cookie" />
    </div>
    <div>
        <h1>抖音</h1>
        <el-input v-model="douYinWssUrl" placeholder="请输入wssUrl" />
    </div>
    <el-button :loading="isStart" @click="startMinecraftLive">启动</el-button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ElMessage } from 'element-plus';
import { onMounted, ref, watch } from 'vue';

const bilibiliRoomId = ref('');
const bilibiliCookie = ref('');
const douYinWssUrl = ref('');
const isStart = ref(false);

onMounted(() => {
    bilibiliRoomId.value = localStorage.getItem('bilibiliRoomId') || '';
    bilibiliCookie.value = localStorage.getItem('bilibiliCookie') || '';
    douYinWssUrl.value = localStorage.getItem('douYinWssUrl') || '';
});

watch(douYinWssUrl, (newValue) => {
    localStorage.setItem('douYinWssUrl', newValue);
});
watch(bilibiliRoomId, (newValue) => {
    localStorage.setItem('bilibiliRoomId', newValue);
});
watch(bilibiliCookie, (newValue) => {
    localStorage.setItem('bilibiliCookie', newValue);
});

const startMinecraftLive = () => {
    isStart.value = true;
    invoke('start_minecraft_live', {
        id: bilibiliRoomId.value,
        cookie: bilibiliCookie.value,
        wss: douYinWssUrl.value,
    }).then(() => {
        ElMessage.success('启动成功');
    }).catch((error) => {
        ElMessage.error(`启动失败: ${error as string}`);
    }).finally(() => {
        isStart.value = false;
    });
};
</script>

<style scoped></style>
