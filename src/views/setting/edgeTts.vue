<template>
    <el-select v-model="currentSpeaker" placeholder="选择边缘TTS发言人" @change="changeEdgeTtsSpeaker">
        <el-option v-for="speaker in edgeTtsSpeakers" :key="speaker" :label="speaker" :value="speaker" />
    </el-select>
    <el-slider v-model="edgeTtsRate" :min="-100" :max="100" @change="changeEdgeTtsRate" />
</template>

<script setup lang="ts">

import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const currentSpeaker = ref('');
let edgeTtsSpeakers = ref([]);
const edgeTtsRate = ref(0);

onMounted(async () => {
    edgeTtsSpeakers.value = await invoke('edge_tts_get_voices');
    edgeTtsRate.value = parseInt(localStorage.getItem('edgeTtsRate') ?? '0');
    currentSpeaker.value = localStorage.getItem('edgeTtsSpeaker') ?? '';
});

const changeEdgeTtsSpeaker = async () => {
    localStorage.setItem('edgeTtsSpeaker', currentSpeaker.value);
};

const changeEdgeTtsRate = async () => {
    localStorage.setItem('edgeTtsRate', edgeTtsRate.value.toString());
};
</script>

<style scoped></style>
