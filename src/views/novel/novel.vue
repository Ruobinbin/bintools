<template>
    <el-tabs tab-position="left">
        <el-tab-pane label="小说">
            <el-text class="mx-1" type="info">长度:{{ novelContents.length }}</el-text>
            <el-input v-model="novelContents" style="width: 100%" autosize type="textarea" placeholder="小说内容" />
        </el-tab-pane>
        <el-tab-pane label="生成音频">
            <el-button @click="setNovelTable">清空并且设置表格</el-button>
            <el-button @click="open(OUTPUT_PATH)">打开音频目录</el-button>
            <el-button @click="insertNovel(-1)">插入</el-button>
            <el-button @click="saveNovel">保存</el-button>
            <el-button @click="generateAllAudio">一键生成所有音频</el-button>
            <br />
            <el-table :data="novels">
                <el-table-column label="索引" :min-width="20">
                    <template #default="scope">
                        {{ scope.$index }}
                    </template>
                </el-table-column>
                <el-table-column prop="content" label="content">
                    <template #default="scope">
                        <el-input v-model="scope.row.content" placeholder="请输入内容"></el-input>
                    </template>
                </el-table-column>
                <el-table-column prop="audioSrc" label="audioSrc">
                    <template #default="scope">
                        <audio :src="convertFileSrc(scope.row.audioSrc)" controls></audio>
                    </template>
                </el-table-column>
                <el-table-column label="操作">
                    <template #default="scope">
                        <el-button @click="generateAudio(scope.$index)" :loading="scope.row.loading">生成音频</el-button>
                        <el-button @click="delNovel(scope.$index)">删除</el-button>
                        <el-button @click="insertNovel(scope.$index)">插入</el-button>
                    </template>
                </el-table-column>
            </el-table>
        </el-tab-pane>
        <el-tab-pane label="gpt-sovits">
            <gpt_sovits ref="gptSovitsRef" />
        </el-tab-pane>
        <el-tab-pane label="视频">
            <audio src=""></audio>
            <div style="display: flex; align-items: center;">
                <el-select v-model="selectedChannel" placeholder="选择博主主页链接" style="flex-grow: 1; margin-right: 10px;">
                    <el-option v-for="channel in channelUrls" :key="channel" :label="channel" :value="channel">
                        {{ channel }}
                    </el-option>
                </el-select>
                <el-button @click="deleteCurrentChannelUrl">删除当前博主链接</el-button>
            </div>
            <el-input v-model="newChannelUrl" placeholder="输入新的博主主页链接"
                @keyup.enter="addNewChannelUrl(newChannelUrl)"></el-input>
            <el-button @click="getVideoList">获取视频列表</el-button>
            <audio ref="audios" controls></audio>
            <el-text>所选视频时长: {{ totalVideoDuration }}</el-text>
            <el-divider>博主视频</el-divider>
            <div>
                <div style="display: flex;flex-wrap: wrap;gap: 10px;">
                    <div :style="{ background: video.selected ? 'yellow' : 'lightgrey', flex: '0 1 auto', width: '200px', borderRadius: '10px' }"
                        v-for="video in videoList" :key="video.id">
                        <img :src="video.getLargestThumbnailUrl()" :alt="video.url"
                            @click="video.selected = !video.selected"
                            style="width: 100%; height: auto; border-radius: 10px;" />
                        <p><el-link type="primary" @click.prevent="open(video.url)">点击此处观看</el-link></p>
                        <p><el-text>ID: {{ video.id }}</el-text></p>
                        <p><el-text>时长: {{ video.duration }} 秒</el-text></p>
                    </div>
                </div>
            </div>
        </el-tab-pane>
        <el-tab-pane label="最后合成">
            <el-steps :active="currentStep" finish-status="success">
                <el-step title="生成字幕文件" />
                <el-step title="下载视频" />
                <el-step title="统一视频格式" />
                <el-step title="合成最终视频" />
            </el-steps>
            <el-button @click="generateVideo">合成视频</el-button>
            <el-select v-model="videoOrientation" placeholder="选择视频方向">
                <el-option label="横屏 (Landscape)" value="landscape"></el-option>
                <el-option label="竖屏 (Portrait)" value="portrait"></el-option>
            </el-select>
            <el-slider v-model="bgmVolume" :min="0" :max="1" :step="0.01" show-stops></el-slider>
            <el-select v-model="selectedBgm" placeholder="选择BGM">
                <el-option v-for="bgm in bgmList" :key="bgm" :label="bgm" :value="bgm"></el-option>
            </el-select>
            <audio v-if="selectedBgm" :src="convertFileSrc(selectedBgm)" controls></audio>
            <el-input v-model="BgmUrl" placeholder="下载BGM" @keyup.enter="downloadBgm(BgmUrl)"></el-input>
        </el-tab-pane>
        <el-tab-pane label="设置">
            <show_database />
        </el-tab-pane>
        <el-tab-pane label="docker日志">
            <docker_log />
        </el-tab-pane>
    </el-tabs>
</template>
<script lang="ts" setup>
import { open } from '@tauri-apps/plugin-shell';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';

import { resourceDir } from '@tauri-apps/api/path';

import gpt_sovits from '../../components/GptSovits.vue';
import show_database from '../../components/ShowDatabase.vue';
import docker_log from '../../components/DockerLog.vue';


import { computed, onMounted, ref } from 'vue'
import { getAudioDuration, getFileNameFromPath } from '../../utils/defaultUtils'
import { Novel } from '../../utils/novelUtils'
import { IThumbnail, Video } from '../../utils/ytdlpUitls'
import { getAllNovels, resetNovelsTable, getAllVideos, getAllChannelUrls, addChannelUrl, deleteChannelUrlByUrl } from '../../utils/dbUtils'
import { ElMessage, ElMessageBox } from 'element-plus'

const OUTPUT_PATH = ref('');//输出路径
let novelContents = ref('') //小说内容
let novels = ref<Novel[]>([])//小说
let videoList = ref<Video[]>([]);//视频列表
let totalVideoDuration = computed(() => {
    return videoList.value
        .filter(video => video.selected)
        .reduce((total, video) => total + video.duration, 0);
});//视频总时长
let channelUrls = ref<string[]>([]); // 博主主页链接列表
let selectedChannel = ref(''); // 当前选中的博主主页链接
let newChannelUrl = ref(''); // 新添加的博主主页链接
const audios = ref<HTMLAudioElement>(); // 所有音频
let videoOrientation = ref('portrait'); // 默认竖屏
let currentStep = ref(0);  // 当前步骤的索引
let bgmList = ref<string[]>([]); // BGM列表
let selectedBgm = ref<string | null>(null); // 选择的BGM
let BgmUrl = ref(''); // 下载BGM的链接
let bgmVolume = ref(0.1); // 默认音量为0.1（10%）
const gptSovitsRef = ref<InstanceType<typeof gpt_sovits> | null>(null);

//载入时触发
onMounted(async () => {
    OUTPUT_PATH.value = await resourceDir() + '\\user_files\\novel_output';
    //载入BGM列表
    fetchBgmList();
    novels.value = await getAllNovels()
    videoList.value = (await getAllVideos()).map(video => {
        let thumbnails: IThumbnail[] = video.thumbnails;
        return new Video(video.id, video.url, video.duration, thumbnails);
    });
    channelUrls.value = await getAllChannelUrls();
    audios.value!.src = convertFileSrc(OUTPUT_PATH.value + '\\audios.wav');
});

//添加博主主页链接
const addNewChannelUrl = async (url: string) => {
    if (url) {
        const success = await addChannelUrl(url);
        if (success) {
            channelUrls.value.push(url);
            ElMessage.success('添加博主主页链接成功');
        } else {
            ElMessage.error('添加博主主页链接失败');
        }
    }
}
//保存小说到数据库
const saveNovel = async () => {
    await resetNovelsTable(novels.value).then(() => {
        ElMessage.success('重置小说数据成功');
    }).catch((error) => {
        ElMessage.error(`重置小说数据失败: ${error as string}`);
    });
}

// 删除当前选中的博主URL
const deleteCurrentChannelUrl = async () => {
    if (selectedChannel.value) {
        await deleteChannelUrlByUrl(selectedChannel.value);
        channelUrls.value = channelUrls.value.filter(channel => channel !== selectedChannel.value);
        selectedChannel.value = '';
    } else {
        ElMessage.warning('请选择一个博主链接');
    }
}
//下载BGM
const downloadBgm = async (url: string) => {
    let path = `/workspace/novel_output/bgm`;
    const cmd = [
        '-x',
        '--audio-format', 'wav',
        '--proxy', 'http://host.docker.internal:7890',
        '-o', `${path}/%(title)s.%(ext)s`,
        url
    ];

    await invoke('run_yt_dlp_cmd', { cmd }).then(() => {
        fetchBgmList();
        ElMessage.success('下载bgm成功');
    }).catch((error) => {
        ElMessage.error(`下载bgm失败: ${error as string}`);
    });
}

// 获取 BGM 列表
const fetchBgmList = async () => {
    const files = await invoke<string[]>('create_dir_and_get_files', { path: `${OUTPUT_PATH.value}\\bgm` });
    bgmList.value = files;
};

const synthesizeAllAudio = async () => {
    try {
        let filePath = `${OUTPUT_PATH.value}\\audios.txt`;
        let text = novels.value.map(novel => `file /workspace/novel_output/${getFileNameFromPath(novel.audioSrc as string)}`).join('\n');
        await invoke('write_string_to_file', { text, filePath });

        let audioSynthesiscmd: string[] = [
            "-y",
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            "/workspace/novel_output/audios.txt",
            "-c:a",
            "pcm_s16le",
            "/workspace/novel_output/audios.wav"
        ];
        await invoke('run_ffmpeg_cmd', { cmd: audioSynthesiscmd });

        audios.value!.src = convertFileSrc(OUTPUT_PATH.value + '\\audios.wav');
        audios.value!.load();
    } catch (error) {
        ElMessage.error(`音频合成失败: ${error as string}`);
    }
};


//获取视频列表
const getVideoList = async () => {
    const cmd = [
        '--flat-playlist',
        '--print-json',
        '--playlist-end', '50',
        selectedChannel.value
    ];

    await invoke('run_yt_dlp_cmd', { cmd }).then((log) => {
        let logStr = log as string;
        videoList.value.push(...logStr.trim().split('\n').map(videoStr => {
            let video = JSON.parse(videoStr);
            let thumbnails: IThumbnail[] = video.thumbnails;
            return new Video(video.id, video.url, video.duration, thumbnails);
        }));
        ElMessage.success('博主视频获取成功');
    }).catch((error) => {
        ElMessage.error(`博主视频获取失败: ${error as string}`);
    });

}
const generateVideo = async () => {
    if (novels.value.some(novel => !novel.audioSrc)) {
        ElMessage.warning('部分音频未生成，请先生成音频');
        return;
    }
    const audioDuration = await getAudioDuration(convertFileSrc(OUTPUT_PATH.value + '\\audios.wav'));
    if (audioDuration > totalVideoDuration.value) {
        ElMessage.warning('视频时长不能小于音频时长');
        return;
    }

    try {
        currentStep.value = 0;
        // 生成字幕所需的txt文件
        let novelsTextFilePath = `${OUTPUT_PATH.value}\\text.txt`;
        let novelsText = novels.value.map(novel => {
            return novel.content
                .replace(/，|。/g, '')
                .split('')
                .join(' ');
        }).join('\n');
        await invoke('write_string_to_file', { text: novelsText, filePath: novelsTextFilePath });

        // 字幕生成
        let audioPath = "/workspace/novel_output/audios.wav";
        let textPath = "/workspace/novel_output/text.txt";
        let outputPath = "/workspace/novel_output/audios.srt";
        await invoke('run_aeneas_cmd', { audioPath, textPath, outputPath });
        currentStep.value = 1;

        // 下载已选视频
        const selected_videos = videoList.value.filter(video => video.selected);
        for (const video of selected_videos) {
            const filePath = `${OUTPUT_PATH.value}\\video\\${video.id}.mp4`;
            const fileExists = await invoke<boolean>('check_file_exists', { path: filePath });
            if (fileExists) {
                continue;
            }
            await video.downloadVideo('/workspace/novel_output/video');
        }
        currentStep.value = 2;

        //统一视频大小
        for (const video of selected_videos) {
            const videoPath = `/workspace/novel_output/video/${video.id}.mp4`;
            const outputPath = `/workspace/novel_output/video/${video.id}_${videoOrientation.value}.mp4`;
            const path = `${OUTPUT_PATH.value}\\video\\${video.id}_${videoOrientation.value}.mp4`
            const fileExists = await invoke<boolean>('check_file_exists', { path: path });
            if (fileExists) {
                continue;
            }
            let cmd = [
                "-y",
                "-i", videoPath,
                '-vf', videoOrientation.value === 'landscape'
                    ? 'scale=-1:1080,pad=1920:1080:(ow-iw)/2:(oh-ih)/2'
                    : 'scale=1080:-1,pad=1080:1920:(ow-iw)/2:(oh-ih)/2',
                '-preset', 'fast',
                outputPath
            ];
            await invoke('run_ffmpeg_cmd', { cmd });
        }

        currentStep.value = 3;

        // 生成用于合成全部视频的videos.txt文件
        let videoListPath = `${OUTPUT_PATH.value}\\videos.txt`;
        let videoPaths = selected_videos.map(video => `file '/workspace/novel_output/video/${video.id}_${videoOrientation.value}.mp4'`).join('\n');
        await invoke('write_string_to_file', { text: videoPaths, filePath: videoListPath });

        const isLandscape = videoOrientation.value === 'landscape';

        let cmd = [
            "-y",
            "-f", "concat",
            "-safe", "0",
            "-i", "/workspace/novel_output/videos.txt",
            "-i", "/workspace/novel_output/audios.wav",
            ...(selectedBgm.value ? ["-stream_loop", "-1", "-i", `/workspace/novel_output/bgm/${getFileNameFromPath(selectedBgm.value)}`] : []),
            ...(selectedBgm.value ? ["-filter_complex", `[2:a]volume=${bgmVolume.value}[bgm]; [1:a][bgm]amix=inputs=2:duration=first[a]`] : []),
            "-vf", `subtitles=/workspace/novel_output/audios.srt:force_style='FontName=Noto Sans CJK SC,FontSize=20,PrimaryColour=&H00FFFF&,WrapStyle=0,Spacing=${isLandscape ? -1 : -3},Alignment=${isLandscape ? 2 : 10}'`,
            ...(selectedBgm.value ? ["-map", "0:v"] : []),
            ...(selectedBgm.value ? ["-map", "[a]"] : []),
            "-c:v", "libx264",
            "-preset", "fast",
            "-c:a", "aac",
            "-shortest",
            "/workspace/novel_output/final_video.mp4"
        ];

        await invoke('run_ffmpeg_cmd', { cmd });

        currentStep.value = 4;

    } catch (error) {
        currentStep.value = 0;
        ElMessage.error(`操作失败: ${error as string}`);
        return;
    }
};

// 生成音频
const generateAudio = async (novelIndex: number) => {
    novels.value[novelIndex].loading = true
    await gptSovitsRef.value?.fetchAudioBlob(novels.value[novelIndex].content).then(async (audioBlob) => {
        novels.value[novelIndex].loading = false
        if (!audioBlob) return;
        const audioData = Array.from(new Uint8Array(await audioBlob.arrayBuffer()));
        const audioName = `audio_${Date.now()}.wav`;
        novels.value[novelIndex].audioSrc = await invoke("save_novel_audio", { audioData, audioName }) as string;
    })
}
//一键生成所有音频
const generateAllAudio = async () => {
    for (let novel of novels.value) {
        if (novel.audioSrc) {
            continue;
        }

        const novelIndex = novels.value.indexOf(novel);
        await generateAudio(novelIndex);
    }
    await synthesizeAllAudio();
};
// 删除小说
const delNovel = (novelIndex: number) => {
    novels.value.splice(novelIndex, 1)
}
// 插入小说
const insertNovel = (novelIndex: number) => {
    novels.value.splice(novelIndex + 1, 0, new Novel());
}

// 设置表格
const setNovelTable = () => {
    ElMessageBox.confirm(
        '此操作会清空表格, 是否继续?',
        'Warning',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning',
        }
    )
        .then(() => {
            if (novelContents.value) {
                novels.value = []
                novelContents.value.split('\n').map((content) => {
                    let novel = new Novel(content)
                    novels.value.push(novel)
                })
            }
        })
        .catch(() => {
            ElMessage.info('已取消');
        })
}



</script>

<style></style>