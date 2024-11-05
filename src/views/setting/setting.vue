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
        <el-tab-pane label="test" name="test">
            <el-input v-model="ytDlpCmd" />
            <el-button @click="runYtDlpCmd">运行yt-dlp命令</el-button>
        </el-tab-pane>
        <el-tab-pane label="edgeTTS" name="edgeTts">
            <EdgeTts />
        </el-tab-pane>
        <el-tab-pane label="Azure TTS" name="azureTts">
            <AzureTts />
        </el-tab-pane>
    </el-tabs>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import ChatApi from '../../components/ChatApi.vue';
import Chat from '../../components/Chat.vue';
import ShowDatabase from '../../components/ShowDatabase.vue';
import EdgeTts from './edgeTts.vue';
import AzureTts from './azureTts.vue';
import { webSocket } from '../../utils/bilibiliLiveUtils';

const activeTab = ref('llmApi');

const ytDlpCmd = ref('');

const runYtDlpCmd = () => {
    console.log(ytDlpCmd.value);
};
onMounted(async () => {
    let cookie = "buvid3=DF900F0C-6939-5789-48F3-7C751C2F8EE045412infoc; b_nut=1710410245; i-wanna-go-back=-1; b_ut=7; _uuid=ECCEA10FF-31012-382B-4F2A-15BFEFD710F5746371infoc; buvid4=0C13F63D-892E-F466-2EB5-6D2A0AB1AC6748532-024031409-9XY8ycWccVDgWVkZ5Fi%2FQQ%3D%3D; rpdid=|(u)~mmuul~J0J'u~u|Ru|km~; hit-dyn-v2=1; LIVE_BUVID=AUTO1917104150894577; buvid_fp_plain=undefined; is-2022-channel=1; CURRENT_BLACKGAP=0; FEED_LIVE_VERSION=V_WATCHLATER_PIP_WINDOW3; enable_web_push=DISABLE; header_theme_version=CLOSE; home_feed_column=5; bp_t_offset_27489840=994569204319387648; fingerprint=ca11c94267fa5b66c8a4f226281de73d; CURRENT_QUALITY=80; SESSDATA=256d7ba9%2C1746221541%2C30d19%2Ab1CjAJlxlCY4K06KCH6u-UsIe0qBE4c-qPROFBYuCHkcoeMniGsEIPDXuF2T7SqTidq4oSVmZ2S2pSZ2I1alFhNjJJLXRWdVctdjllV1Z2V2VETkwwcGlDU2ZyT1dKQWY1YmZwS2dHZEpaUjFIRWxZTkt6aC01RlFCUkNEeElYaUxwQUIyTmZPbU13IIEC; bili_jct=5db757fea8ed46e4f24d185e1e44f32a; DedeUserID=3493293971212575; DedeUserID__ckMd5=08851db8bd8cdd6e; browser_resolution=1912-920; b_lsid=C9A1A824_192F88BE3D9; bsource=search_bing; bili_ticket=eyJhbGciOiJIUzI1NiIsImtpZCI6InMwMyIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MzEwMDYwMTQsImlhdCI6MTczMDc0Njc1NCwicGx0IjotMX0.CpZENbqeifSWoPQRidkVcfOh2dZrzoH1phdRACDAV80; bili_ticket_expires=1731005954; buvid_fp=ca11c94267fa5b66c8a4f226281de73d; CURRENT_FNVAL=4048; bp_t_offset_3493293971212575=996047583602278400; Hm_lvt_8a6e55dbd2870f0f5bc9194cddf32a02=1730676225,1730685184,1730731213,1730748905; HMACCOUNT=5710C3BC38D8A87C; Hm_lpvt_8a6e55dbd2870f0f5bc9194cddf32a02=1730749108; sid=mxvw213t; PVID=4"
    await webSocket(27851594, cookie);
});
</script>

<style scoped></style>
