<template>
    <el-input v-model="key" placeholder="输入Azure TTS密钥" />
    <el-button @click="saveKey">保存</el-button>
    <el-select v-model="currentSpeaker" placeholder="选择说话人" @change="changeSpeaker">
        <el-option v-for="speaker in speakers" :key="speaker.shortName" :label="speaker.localName"
            :value="speaker.shortName" />
    </el-select>
    <el-slider :step="0.1" v-model="rate" :min="0" :max="2" @change="changeRate" />
</template>

<script setup lang="ts">

import { onMounted, ref } from 'vue';
import { getSpeakers, Speaker } from '../../utils/azureTtsUtils';

const currentSpeaker = ref('');
const speakers = ref<Speaker[]>([]);
const rate = ref(0);
const key = ref('');
onMounted(async () => {
    speakers.value = await getSpeakers();
    rate.value = parseFloat(localStorage.getItem('azureTtsRate') ?? '1');
    key.value = localStorage.getItem('azureTtsKey') ?? '';
    currentSpeaker.value = localStorage.getItem('azureTtsSpeaker') ?? '';
});

const saveKey = async () => {
    localStorage.setItem('azureTtsKey', key.value);
};

const changeSpeaker = async () => {
    localStorage.setItem('azureTtsSpeaker', currentSpeaker.value);
};

const changeRate = async () => {
    localStorage.setItem('azureTtsRate', rate.value.toString());
};
</script>

<style scoped></style>
