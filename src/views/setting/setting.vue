<template>
    <el-tabs v-model="activeTab">
        <el-tab-pane label="LLM API" name="llmApi">
            <ChatApi />
            <el-divider>Chat</el-divider>
            <Chat />
        </el-tab-pane>
        <el-tab-pane label="数据库" name="db">
            <ShowDatabase />
        </el-tab-pane>
    </el-tabs>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import ChatApi from '../../components/ChatApi.vue';
import Chat from '../../components/Chat.vue';
import ShowDatabase from '../../components/ShowDatabase.vue';

const activeTab = ref('llmApi');

onMounted(() => {
});
</script>

<style scoped></style>
