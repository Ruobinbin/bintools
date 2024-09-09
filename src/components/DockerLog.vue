<template>
    <el-scrollbar ref="scrollbar" style="max-height: 300px; overflow-y: auto; margin-bottom: 10px;">
        <p v-for="log in logs" :key="log">{{ log }}</p>
    </el-scrollbar>
    <el-button @click="clearLogs" type="danger">清空日志</el-button>
</template>

<script lang="ts" setup>
import { ref, onMounted, nextTick } from 'vue';
import { listen } from '@tauri-apps/api/event';

const logs = ref<string[]>([]);
const scrollbar = ref();

onMounted(async () => {
    listen('gpt_sovits_api_log', async (event) => {
        logs.value.push(event.payload as string);
        await nextTick(); // 等待DOM更新完成
        scrollToBottom();
    });
});

const scrollToBottom = () => {
    // 确保 scrollbar 存在且已渲染
    if (scrollbar.value) {
        scrollbar.value.scrollTo({ top: scrollbar.value.$el.scrollHeight });
    }
};

const clearLogs = () => {
    logs.value = [];
};
</script>

<style></style>