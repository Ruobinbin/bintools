<template>
    <el-tabs tab-position="left">
        <el-tab-pane label="小说">
            <el-text class="mx-1" type="info">长度:{{ novelContents.length }}</el-text>
            <el-button @click="formatNovel" :loading="isFormatting">格式化</el-button>
            <el-button @click="edgeTtsGenerateAllAudio" :loading="isEdgeTtsGenerating">edgeTts生成音频</el-button>
            <audio :src="`${convertFileSrc(audiosSrc)}?t=${new Date().getTime()}`" controls></audio>
            <el-input v-model="novelContents" style="width: 100%;" autosize type="textarea" placeholder="小说内容" />
        </el-tab-pane>
        <el-tab-pane label="生成音频">
            <el-button @click="formatNovelToJson" :loading="isFormatting">chat格式化小说</el-button>
            <el-button @click="formatNovel" :loading="isFormatting">格式化小说</el-button>
            <el-button @click="open(OUTPUT_PATH)">打开输出目录</el-button>
            <el-button @click="insertNovel(-1)">插入</el-button>
            <el-button @click="saveNovel" :loading="isResetting">保存</el-button>
            <el-button @click="generateAllAudio">一键生成所有音频</el-button>
            <el-button @click="edgeTtsGenerateAllAudio">edgeTts一键生成所有音频</el-button>
            <el-button @click="splitNovel">手动分离：“</el-button>
            <el-button @click="clearAllAudio">一键清空所有音频</el-button>
            <el-divider>人物设置</el-divider>
            <el-button @click="addSpeakerDescription">添加</el-button>
            <el-table :data="speakerDescriptionsList">
                <el-table-column prop="speaker" label="speaker">
                    <template #default="scope">
                        <el-input v-model="scope.row.speaker" placeholder="输入说话人"></el-input>
                    </template>
                </el-table-column>
                <el-table-column prop="modelName" label="modelName">
                    <template #default="scope">
                        <el-select v-model="scope.row.modelName" placeholder="选择模型"
                            @change="setSpeakerModel(scope.row.speaker, $event)">
                            <el-option v-for="model in gptSovitsModels" :key="model.model_name"
                                :label="model.model_name" :value="model.model_name"></el-option>
                        </el-select>
                    </template>
                </el-table-column>
                <el-table-column prop="description" label="描述">
                    <template #default="scope">
                        <el-input v-model="scope.row.description" placeholder="输入描述"></el-input>
                    </template>
                </el-table-column>
            </el-table>
            <el-divider>小说</el-divider>
            <el-table :data="currentPageData">
                <el-table-column prop="speaker" label="speaker">
                    <template #default="scope">
                        <el-select v-model="scope.row.speaker" placeholder="选择说话人"
                            @change="speakerChanged(scope.row, $event)">
                            <el-option v-for="speakerDescription in speakerDescriptionsList"
                                :key="speakerDescription.speaker" :label="speakerDescription.speaker"
                                :value="speakerDescription.speaker">
                            </el-option>
                        </el-select>
                    </template>
                </el-table-column>
                <el-table-column prop="model" label="model">
                    <template #default="scope">
                        <el-tag v-if="!scope.row.model" type="danger">未设置</el-tag>
                        <el-tag v-else>{{ scope.row.model }}</el-tag>
                    </template>
                </el-table-column>
                <el-table-column prop="emotion" label="emotion">
                    <template #default="scope">
                        <el-input v-model="scope.row.emotion" placeholder="输入情感"></el-input>
                    </template>
                </el-table-column>
                <el-table-column prop="refAudioPath" label="refAudioPath">
                    <template #default="scope">
                        <el-select v-model="scope.row.refAudioPath" placeholder="选择参考音频路径">
                            <el-option v-for="audioPath in getRefAudiosPathByNovel(scope.row)" :key="audioPath"
                                :label="audioPath" :value="audioPath">
                            </el-option>
                        </el-select>
                        <audio v-if="scope.row.refAudioPath" :src="convertFileSrc(scope.row.refAudioPath)"
                            controls></audio>
                    </template>
                </el-table-column>
                <el-table-column prop="content" label="content">
                    <template #default="scope">
                        <el-input v-model="scope.row.content" autosize type="textarea" placeholder="请输入内容"
                            :input-style="{ color: isTextContained(scope.row.content) ? 'red' : 'black' }"></el-input>
                    </template>
                </el-table-column>
                <el-table-column prop="audioSrc" label="audioSrc">
                    <template #default="scope">
                        <audio v-if="scope.row.audioSrc" :src="convertFileSrc(scope.row.audioSrc)" controls></audio>
                    </template>
                </el-table-column>
                <el-table-column label="操作">
                    <template #default="scope">
                        <el-button @click="generateAudio(scope.row)" :loading="scope.row.loading">生成音频</el-button>
                        <el-button @click="edgeTtsGenerateAudio(scope.row)"
                            :loading="scope.row.loading">edgeTts生成音频</el-button>
                        <el-button @click="delNovel(scope.$index)">删除</el-button>
                        <el-button @click="insertNovel(scope.$index)">插入</el-button>
                    </template>
                </el-table-column>
            </el-table>
            <el-pagination background layout="prev, pager, next" :total="novels.length" :page-size="pageSize"
                :current-page.sync="currentPage" @current-change="handlePageChange" />
        </el-tab-pane>
        <el-tab-pane label="gpt-sovits">
            <GptSovits />
        </el-tab-pane>
        <el-tab-pane label="视频">
            <audio src=""></audio>
            <div style="display: flex; align-items: center;">
                <el-select v-model="selectedChannel" placeholder="选择博主主页链接" style="flex-grow: 1; margin-right: 10px;">
                    <el-option v-for="channel in channelUrls" :key="channel" :label="channel" :value="channel">
                        {{ channel }}
                    </el-option>
                </el-select>
                <el-button type="primary" @click="addNewChannelUrl">添加</el-button>
                <el-button type="danger" @click="deleteCurrentChannelUrl">删除</el-button>
            </div>
            <el-button @click="open(OUTPUT_PATH)">打开输出目录</el-button>
            <el-button @click="getVideoList">获取视频列表</el-button>
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
                        <el-button @click="delVideo(video.id)">删除</el-button>
                        <el-button @click="video.downloadVideo('/workspace/novel_output/video')"
                            :loading="video.isDownloading">下载</el-button>
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
            <el-button @click="open(OUTPUT_PATH)">打开输出目录</el-button>
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
        <el-tab-pane label="docker日志">
            <DockerLog />
        </el-tab-pane>
    </el-tabs>
</template>
<script lang="ts" setup>
import { open } from '@tauri-apps/plugin-shell';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { resourceDir } from '@tauri-apps/api/path';
import { getAudioDuration, getFileNameFromPath } from '../../utils/defaultUtils'
import { IGptSovitsModel, ITTSRequestParams, fetchAudioBlob, getGptSovitsModels, localPathToContainerPath, setGptModel, setSovitsModel, isGptSovitsStart } from '../../utils/gptSovitsUtils'

import { Novel } from '../../utils/novelUtils'
import { IThumbnail, Video } from '../../utils/ytdlpUitls'
import { getAllNovels, resetNovelsTable, getAllVideos, getAllChannelUrls, addChannelUrl, deleteChannelUrlByUrl, deleteVideoById, getAllSpeakerDescriptions, resetSpeakerDescriptions } from '../../utils/dbUtils'
import { ElButton, ElMessage, ElMessageBox } from 'element-plus';
import GptSovits from '../../components/GptSovits.vue';
import DockerLog from '../../components/DockerLog.vue';
import { computed, onMounted, ref } from 'vue';
import { zodResponseFormat } from "openai/helpers/zod";
import { z } from "zod";

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
let audiosSrc = ref(''); // 所有音频路径
let videoOrientation = ref('portrait'); // 默认竖屏
let currentStep = ref(0);  // 当前步骤的索引
let bgmList = ref<string[]>([]); // BGM列表
let selectedBgm = ref<string>(''); // 选择的BGM
let BgmUrl = ref(''); // 下载BGM的链接
let bgmVolume = ref(0.1); // 默认音量为0.1（10%）
let isFormatting = ref(false); // 是否正在格式化
let gptSovitsModels = ref<IGptSovitsModel[]>([]); // gpt-sovits模型列表
let currentModelName = ref<string>(''); // 当前模型
let currentPage = ref(1); // 当前页
const pageSize = ref(6);
let currentPageData = computed(() => novels.value.slice((currentPage.value - 1) * pageSize.value, currentPage.value * pageSize.value)); // 当前页的小说
let speakerDescriptionsList = ref<{ speaker: string, description: string, modelName: string }[]>([]); // 说话人描述列表
let isResetting = ref(false); // 是否正在重置
let isEdgeTtsGenerating = ref(false); // 是否正在edgeTts生成


//载入时触发
onMounted(async () => {
    OUTPUT_PATH.value = (await resourceDir()) + '\\user_files\\novel_output';
    fetchBgmList();
    novels.value = await getAllNovels();
    speakerDescriptionsList.value = await getAllSpeakerDescriptions();
    gptSovitsModels.value = await getGptSovitsModels();
    videoList.value = (await getAllVideos()).map(video => {
        let thumbnails: IThumbnail[] = video.thumbnails;
        return new Video(video.id, video.url, video.duration, thumbnails);
    });
    channelUrls.value = await getAllChannelUrls();
    audiosSrc.value = OUTPUT_PATH.value + '\\audios.wav';
});

const addSpeakerDescription = () => {
    speakerDescriptionsList.value.push({ speaker: '', description: '', modelName: '' });
}

const formatNovel = () => {
    novelContents.value = novelContents.value
        .replace(/[“「]/g, '【')    // 替换左引号
        .replace(/[”」]/g, '】')    // 替换右引号
        // 使用负向前瞻，避免替换【】中的标点符号
        .replace(/[:：,，。](?![^【】]*】)/g, '\n')
        .replace(/】/g, '】\n')
        .replace(/\n\s*\n/g, '\n'); // 替换连续的换行符为一个换行符
};

const clearAllAudio = async () => {
    for (const novel of novels.value) {
        novel.audioSrc = '';
    }
    ElMessage.success('清空所有音频成功');
}

const handlePageChange = (page: number) => {
    currentPage.value = page;
};

const isTextContained = (text: string): boolean => {
    const regex = /[:：]["“‘']/;
    const result = regex.test(text);
    return result;
};

const splitNovel = () => {
    novels.value.filter(novel => isTextContained(novel.content)).forEach(novel => {
        let [part1, part2] = novel.content.split(/[:：]/);
        novel.content = part2;
        let newNovel = new Novel({ content: part1, speaker: '旁白' });
        novels.value.splice(novels.value.indexOf(novel), 0, newNovel);
    });
}

const speakerChanged = (novel: Novel, speaker: string) => {
    const modelName = speakerDescriptionsList.value.find(description => description.speaker === speaker)?.modelName;

    if (modelName) {
        novel.model = modelName;
        const model = gptSovitsModels.value.find(m => m.model_name === modelName);
        if (model) {
            const refAudios = model.ref_audios.filter(audio => audio.emotion === novel.emotion || audio.emotion === "平静");
            if (refAudios.length > 0) {
                const randomIndex = Math.floor(Math.random() * refAudios.length);
                novel.refAudioPath = refAudios[randomIndex].path;
            }
        }

    }
}

//设置说话人模型
const setSpeakerModel = (speaker: string, modelName: string) => {
    const speakerNovels = novels.value.filter(novel => novel.speaker === speaker);
    speakerNovels.forEach(novel => {
        novel.model = modelName;
        const model = gptSovitsModels.value.find(m => m.model_name === novel.model);
        if (model) {
            const refAudios = model.ref_audios.filter(audio => audio.emotion === novel.emotion || audio.emotion === "平静");
            if (refAudios.length > 0) {
                const randomIndex = Math.floor(Math.random() * refAudios.length);
                novel.refAudioPath = refAudios[randomIndex].path;
            }
        }
    });
};

//获取说话人模型对应的参考音频路径
const getRefAudiosPathByNovel = (novel: Novel) => {
    const model = gptSovitsModels.value.find(m => m.model_name === novel.model);
    if (model) {
        const refAudios = model.ref_audios.filter(audio => audio.emotion === novel.emotion);
        if (refAudios.length > 0) {
            return refAudios.map(audio => audio.path);
        }
        return model.ref_audios.map(audio => audio.path);
    }
    return [];
};

//格式化为json
const formatNovelToJson = async () => {
    isFormatting.value = true;
    if (!novelContents.value) {
        ElMessage.warning('小说内容不能为空');
        isFormatting.value = false;
        return;
    }

    const emotion = z.enum(['平静', '愤怒', '高兴', '悲伤', '惊讶', '恐惧', '厌恶', '困惑', '紧张']);

    const novel = z.object({
        content: z.string(),
        speaker: z.string(),
        emotion: emotion,
    });

    const speakerDescriptions = z.object({
        speaker: z.string(),
        description: z.string(),
    });

    const novelResponse = z.object({
        speakerDescriptions: z.array(speakerDescriptions),
        novels: z.array(novel),
    });

    let chatApiUrl = localStorage.getItem('chatApiUrl');
    if (!chatApiUrl) {
        ElMessage.warning('未设置caht API URL');
        return;
    }
    let chatApiKey = localStorage.getItem('chatApiKey') ?? ""
    let chatModel = localStorage.getItem('chatModel') ?? ""

    const messages = [
        {
            role: 'system',
            content: `为小说中的每个人物设置简介,即便是没有名字的人物也要,简介最好说明性别。
            理解小说后根据上下文判断每个编号后内容的说话人物是谁以及情感,并设置content,speaker,emotion。
            emotion必须从以下选择: ${emotion.options.join(', ')},没有就设置为平静,不要设置为选项中没有的!
            【】这俩符号中间的内容是小说人物说的话。
            旁白也是speaker,emotion为平静。`
        },
        {
            role: 'user',
            content: novelContents.value
        }
    ];

    try {
        const response = await fetch(`${chatApiUrl}/v1/chat/completions`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${chatApiKey}`
            },
            body: JSON.stringify({
                model: chatModel,
                messages: messages,
                response_format: zodResponseFormat(novelResponse, "novelResponse"),
            })
        });
        if (!response.ok) {
            throw new Error(`格式化小说失败: ${response.status}`);
        }
        const data = await response.json();
        console.log(data)
        const responseJson = JSON.parse(data.choices[0].message.content ?? '');
        speakerDescriptionsList.value = responseJson.speakerDescriptions;
        novels.value = responseJson.novels.map((novel: any) => ({
            content: novel.content,
            speaker: novel.speaker,
            emotion: novel.emotion,
        }));
    } catch (error) {
        ElMessage.error(`格式化小说失败: ${error as string}`);
    } finally {
        isFormatting.value = false;
    }
}

//添加博主主页链接
const addNewChannelUrl = async () => {
    const { value: url } = await ElMessageBox.prompt('请输入新的博主主页链接', '添加博主主页链接', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
    });

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
    isResetting.value = true;
    try {
        await resetNovelsTable(novels.value);
        await resetSpeakerDescriptions(speakerDescriptionsList.value);
        ElMessage.success('重置成功');
    } catch (error) {
        ElMessage.error(`重置数据失败: ${error as string}`);
    } finally {
        isResetting.value = false;
    }
}

//删除视频
const delVideo = async (id: string) => {
    await deleteVideoById(id);
    videoList.value = videoList.value.filter(video => video.id !== id);
}

// 删除当前选中的博主URL
const deleteCurrentChannelUrl = async () => {
    if (selectedChannel.value) {
        await deleteChannelUrlByUrl(selectedChannel.value);
        channelUrls.value = channelUrls.value.filter(channel => channel !== selectedChannel.value);
        selectedChannel.value = '';
        ElMessage.success('删除博主主页链接成功');
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

    if (novels.value.some(novel => !novel.audioSrc)) {
        ElMessage.warning('部分音频未生成，请先生成音频');
        return;
    }

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

        audiosSrc.value = OUTPUT_PATH.value + '\\audios.wav';
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

    try {
        const audioDuration = await getAudioDuration(convertFileSrc(OUTPUT_PATH.value + '\\audios.wav'));
        if (audioDuration > totalVideoDuration.value) {
            ElMessage.warning('视频时长不能小于音频时长');
            return;
        }

        currentStep.value = 0;
        // 生成字幕所需的txt文件
        let novelsTextFilePath = `${OUTPUT_PATH.value}\\text.txt`;
        await invoke('write_string_to_file', { text: novelContents.value, filePath: novelsTextFilePath });

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
            "-vf", `subtitles=/workspace/novel_output/audios.srt:force_style='FontName=Noto Sans CJK SC,FontSize=12,PrimaryColour=&H00FFFF&,OutlineColour=&H000000&,Outline=1,WrapStyle=0,Spacing=-1,Alignment=${isLandscape ? 2 : 10}'`,
            "-map", "0:v",
            ...(selectedBgm.value ? ["-map", "[a]"] : ["-map", "1:a"]),
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

const edgeTtsGenerateAudio = async (novel: Novel) => {
    novel.loading = true;
    const speaker = localStorage.getItem('edgeTtsSpeaker');
    if (!speaker) {
        ElMessage.error('未设置edge说话人');
        return;
    }
    const rate = parseInt(localStorage.getItem('edgeTtsRate') ?? "0")
    const audioPath = `${OUTPUT_PATH.value}\\audio_${Date.now()}.wav`
    const text = novel.content
    await invoke('edge_tts', { audioPath, speaker, rate, text })
        .then(() => {
            novel.audioSrc = audioPath;
        })
        .catch(error => {
            ElMessage.error(`生成音频失败: ${error as string}`);
        })
        .finally(() => {
            novel.loading = false;
        });
}
const generateAudio = async (novel: Novel) => {
    novel.loading = true;
    try {
        if (!isGptSovitsStart) {
            throw new Error('GPT Sovits 未启动，请先启动');
        }

        if (!novel.speaker || !novel.model) {
            throw new Error('未设置说话人模型');
        }

        const model = gptSovitsModels.value.find(m => m.model_name === novel.model);
        if (!model) {
            throw new Error(`未找到对应的模型: ${novel.model}`);
        }

        const selectedRefAudio = model.ref_audios.find(audio => audio.path === novel.refAudioPath);
        if (!selectedRefAudio) {
            throw new Error('参考音频不存在');
        }

        let requestParams: ITTSRequestParams = {
            "text": novel.content,                   // str. (必填) 要合成的文本
            "text_lang": "zh",              // str. (必填) 要合成文本的语言
            "ref_audio_path": localPathToContainerPath(selectedRefAudio.path),         // str. (必填) 参考音频路径
            "aux_ref_audio_paths": [],    // list. (可选) 辅助参考音频路径，用于多说话人音调融合
            "prompt_text": selectedRefAudio.content,            // str. (可选) 参考音频的提示文本
            "prompt_lang": selectedRefAudio.language,            // str. (必填) 参考音频提示文本的语言
            "top_k": 5,                   // int. top k 采样
            "top_p": 1,                   // float. top p 采样
            "temperature": 1,             // float. 采样的温度
            "text_split_method": "cut5",  // str. 文本拆分方法，参见 text_segmentation_method.py 了解详细信息
            "batch_size": 1,              // int. 推理时的批处理大小
            "batch_threshold": 0.75,      // float. 批处理拆分的阈值
            "split_bucket": true,         // bool. 是否将批处理拆分成多个桶
            "speed_factor": 1.0,           // float. 控制合成音频的速度
            "streaming_mode": false,      // bool. 是否返回流式响应
            "seed": -1,                   // int. 随机种子，用于复现
            "parallel_infer": true,       // bool. 是否使用并行推理
            "repetition_penalty": 1.35    // float. T2S 模型的重复惩罚
        }

        if (currentModelName.value !== model.model_name) {
            await setGptModel(localPathToContainerPath(model.gpt_model_paths[0]));
            await setSovitsModel(localPathToContainerPath(model.sovits_model_paths[0]));
            currentModelName.value = model.model_name;
            ElMessage.success('切换模型成功');
        }

        await fetchAudioBlob(requestParams).then(async (audioBlob) => {
            const audioData = Array.from(new Uint8Array(await audioBlob.arrayBuffer()));
            const audioName = `audio_${Date.now()}.wav`;
            novel.audioSrc = await invoke("save_novel_audio", { audioData, audioName }) as string;
        })
    } catch (error) {
        ElMessage.error(`${error as string}`);
        throw error;
    } finally {
        novel.loading = false;
    }
}

//一键生成所有音频
const generateAllAudio = async () => {
    for (const novel of novels.value) {
        if (novel.audioSrc) {
            continue;
        }

        try {
            await generateAudio(novel);
        } catch (error) {
            return;
        }
    }
    await synthesizeAllAudio();
};

const edgeTtsGenerateAllAudio = async () => {
    isEdgeTtsGenerating.value = true;
    const speaker = localStorage.getItem('edgeTtsSpeaker');
    if (!speaker) {
        ElMessage.error('未设置edge说话人');
        isEdgeTtsGenerating.value = false;
        return;
    }
    const rate = parseInt(localStorage.getItem('edgeTtsRate') ?? "0")
    const audioPath = `${OUTPUT_PATH.value}\\audios.wav`
    const text = novelContents.value
    await invoke('edge_tts', { audioPath, speaker, rate, text })
        .then(() => {
            ElMessage.success('生成音频成功');
            audiosSrc.value = OUTPUT_PATH.value + '\\audios.wav';
        })
        .catch(error => {
            ElMessage.error(`生成音频失败: ${error as string}`);
        })
        .finally(() => {
            isEdgeTtsGenerating.value = false;
        });
}

// 删除小说
const delNovel = (novelIndex: number) => {
    const globalIndex = (currentPage.value - 1) * pageSize.value + novelIndex;
    novels.value.splice(globalIndex, 1)
}
// 插入小说
const insertNovel = (novelIndex: number) => {
    const globalIndex = (currentPage.value - 1) * pageSize.value + novelIndex;
    novels.value.splice(globalIndex + 1, 0, new Novel());
}

</script>

<style></style>