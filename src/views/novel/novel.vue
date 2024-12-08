<template>
    <el-tabs tab-position="left">
        <el-tab-pane label="小说">
            <el-text type="info">长度:{{ novelContents.length }}</el-text>
            <div flex items-center>
                <el-input w-md v-model="novelUrl" placeholder="小说链接" />
                <el-button @click="getZhihuNovel">打开知乎小说</el-button>
            </div>
            <el-button @click="removeNumberedLines">去除编号</el-button>
            <el-button @click="replaceNewlineWithEnter">替换\n为换行</el-button>
            <el-input v-model="novelContents" h-100 w-full type="textarea" placeholder="小说内容" />
        </el-tab-pane>
        <el-tab-pane label="视频">
            <VideoList @updateVideoList="handleVideoListUpdate"
                @updateCurrentVideoList="handleCurrentVideoListUpdate" />
        </el-tab-pane>
        <el-tab-pane label=" 最后合成">
            <div>
                <el-progress type="circle" :percentage="generateAudioPercentage">
                    <template #default="{ percentage }">
                        生成音频{{ percentage }}%
                    </template>
                </el-progress>
                <el-progress type="circle" :percentage="aeneasPercentage">
                    <template #default="{ percentage }">
                        生成字幕{{ percentage }}%
                    </template>
                </el-progress>
                <el-progress type="circle" :percentage="formatVideoPercentage">
                    <template #default="{ percentage }">
                        统一格式{{ percentage }}%
                    </template>
                </el-progress>
                <el-progress type="circle" :percentage="synthesizeVideoPercentage">
                    <template #default="{ percentage }">
                        合成视频{{ percentage }}%
                    </template>
                </el-progress>
            </div>
            <el-divider>操作</el-divider>
            <el-button @click="generateVideo" :loading="isVideoGenerating">合成视频</el-button>
            <el-button @click="open(OUTPUT_PATH)">打开输出目录</el-button>
            <el-divider>设置</el-divider>
            <div w-md>
                <el-radio-group v-model="ttsOption">
                    <el-radio value="edgetts">edgeTts</el-radio>
                    <el-radio value="azuretts">azureTts</el-radio>
                </el-radio-group>
                <el-input v-model="novelName" placeholder="小说名" />
                <el-input v-model="novelIntro" type="textarea" placeholder="小说介绍" h-25 />
                <el-select v-model="videoOrientation" placeholder="选择视频方向">
                    <el-option label="横屏 (Landscape)" value="landscape"></el-option>
                    <el-option label="竖屏 (Portrait)" value="portrait"></el-option>
                </el-select>
                <div style="display: flex; align-items: center;">
                    <el-checkbox v-model="isIncludeVideoAudio" style="flex-grow: 1;">包含视频音频</el-checkbox>
                    <el-slider v-model="videoAudioVolume" :min="0" :max="1" :step="0.01" show-stops
                        v-if="isIncludeVideoAudio" style="flex-grow: 1;"></el-slider>
                </div>
                <div>
                    <el-select v-model="selectedBgm" placeholder="选择BGM">
                        <el-option v-for=" bgm in bgmList" :key="bgm" :label="bgm" :value="bgm"></el-option>
                    </el-select>
                    <div flex items-center>
                        <el-input v-model="BgmUrl" placeholder="下载BGM"></el-input>
                        <el-button @click="downloadBgm(BgmUrl)">下载BGM</el-button>
                    </div>
                    <audio v-if="selectedBgm" :src="convertFileSrc(selectedBgm)" controls></audio>
                    <el-slider v-if="selectedBgm" v-model="bgmVolume" :min="0" :max="1" :step="0.01" show-stops
                        inline></el-slider>
                </div>
            </div>
            <el-divider>上传</el-divider>
            <div flex items-center w-md>
                <el-input v-model="tagInput" placeholder="输入标签并按回车" @keyup.enter="addTag"
                    @keydown.backspace="removeLastTag">
                </el-input>
                <el-tag v-for="(tag, index) in tags" :key="index" closable @close="removeTag(index)">
                    {{ tag }}
                </el-tag>
            </div>
            <div flex items-center w-md>
                <el-input v-model="videoPath" placeholder="视频路径" readonly></el-input>
                <el-button @click="selectFile">选择文件</el-button>
            </div>
            <div>
                <el-checkbox v-model="isBilibili">b站</el-checkbox>
                <el-checkbox v-model="isDouyin">抖音</el-checkbox>
                <el-checkbox v-model="isKuaishou">快手</el-checkbox>
                <el-checkbox v-model="isBaidu">百度</el-checkbox>
                <el-checkbox v-model="isDayuhao">大鱼号</el-checkbox>
                <el-button @click="upload()" :loading="isUpload">一键上传</el-button>
            </div>
            <audio :src="`${convertFileSrc(audiosSrc)}?t=${new Date().getTime()}`" controls></audio>
            <video h-100 :src="`${convertFileSrc(videoPath)}?t=${new Date().getTime()}`" controls />
        </el-tab-pane>
        <el-tab-pane label=" docker日志">
            <DockerLog />
        </el-tab-pane>
    </el-tabs>
</template>
<script lang="ts" setup>
import { open } from '@tauri-apps/plugin-shell';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { resourceDir } from '@tauri-apps/api/path';
import { getAudioDuration, getFileNameFromPath } from '../../utils/defaultUtils'
import { Video } from '../../utils/ytdlpUitls'
import { ElButton, ElMessage } from 'element-plus';
import DockerLog from '../../components/DockerLog.vue';
import VideoList from './video.vue';
import { onMounted, ref, watch } from 'vue';
import { generateCompleteAudioData } from '../../utils/azureTtsUtils';

const OUTPUT_PATH = ref('');//输出路径
let novelContents = ref('') //小说内容
let videoList = ref<Video[]>([]);//视频列表
let currentVideoList = ref<Video[]>([]);//当前视频列表
let audiosSrc = ref(''); // 所有音频路径
let videoOrientation = ref('portrait'); // 默认竖屏
let bgmList = ref<string[]>([]); // BGM列表
let selectedBgm = ref<string>(''); // 选择的BGM
let BgmUrl = ref(''); // 下载BGM的链接
let bgmVolume = ref(0.1); // 默认音量为0.1（10%）
let isEdgeTtsGenerating = ref(false); // 是否正在edgeTts生成
let isAzureTtsGenerating = ref(false); // 是否正在Azure TTS生成
let isIncludeVideoAudio = ref(false); // 是否包含视频音频
let isVideoGenerating = ref(false); // 是否正在生成视频
let videoAudioVolume = ref(0.1); // 视频音频音量
let novelName = ref(''); // 小说名
let novelIntro = ref(''); // 小说介绍
let aeneasPercentage = ref(0)
let formatVideoPercentage = ref(0); // 统一格式进度
let synthesizeVideoPercentage = ref(0); // 合成视频进度
let generateAudioPercentage = ref(0); // 生成音频进度
let videoPath = ref("");
let isUpload = ref(false);
let tagInput = ref('');
let tags = ref<string[]>([]);
let novelUrl = ref('');
let isBilibili = ref(false);
let isDouyin = ref(false);
let isKuaishou = ref(false);
let isBaidu = ref(false);
let isDayuhao = ref(false);
let ttsOption = ref('edgetts');

//载入时触发
onMounted(async () => {
    OUTPUT_PATH.value = (await resourceDir()) + '\\user_files\\novel_output';
    audiosSrc.value = OUTPUT_PATH.value + '\\audios.wav';
    fetchBgmList();
    videoAudioVolume.value = parseFloat(localStorage.getItem('videoAudioVolume') || '0.1');
    bgmVolume.value = parseFloat(localStorage.getItem('bgmVolume') || '0.5');
    novelContents.value = localStorage.getItem('novelContents') || '';
    novelName.value = localStorage.getItem('novelName') || '';
    novelIntro.value = localStorage.getItem('novelIntro') || '';
    videoPath.value = localStorage.getItem("uploadVideoPath") || "";
    tags.value = JSON.parse(localStorage.getItem('tags') || '[]');
    ttsOption.value = localStorage.getItem('ttsOption') || 'edgetts';
    // 初始化平台布尔值
    isBilibili.value = localStorage.getItem('isBilibili') === 'true';
    isDouyin.value = localStorage.getItem('isDouyin') === 'true';
    isKuaishou.value = localStorage.getItem('isKuaishou') === 'true';
    isBaidu.value = localStorage.getItem('isBaidu') === 'true';
    isDayuhao.value = localStorage.getItem('isDayuhao') === 'true';
});

watch(videoAudioVolume, (newVolume) => {
    localStorage.setItem('videoAudioVolume', newVolume.toString());
});
watch(bgmVolume, (newVolume) => {
    localStorage.setItem('bgmVolume', newVolume.toString());
});
watch(novelContents, (newContents) => {
    localStorage.setItem('novelContents', newContents);
});
watch(novelName, (newName) => {
    localStorage.setItem('novelName', newName);
});
watch(novelIntro, (newIntro) => {
    localStorage.setItem('novelIntro', newIntro);
});
watch(isBilibili, (newValue) => {
    localStorage.setItem('isBilibili', newValue.toString());
});
watch(isDouyin, (newValue) => {
    localStorage.setItem('isDouyin', newValue.toString());
});
watch(isKuaishou, (newValue) => {
    localStorage.setItem('isKuaishou', newValue.toString());
});
watch(isBaidu, (newValue) => {
    localStorage.setItem('isBaidu', newValue.toString());
});
watch(isDayuhao, (newValue) => {
    localStorage.setItem('isDayuhao', newValue.toString());
});
watch(ttsOption, (newValue) => {
    localStorage.setItem('ttsOption', newValue);
});


function removeNumberedLines() {
    novelContents.value = novelContents.value.replace(/^\d+\s*$/gm, '');
    novelContents.value = novelContents.value.replace(/^\s*[\r\n]/gm, '');
    novelContents.value = novelContents.value.replace(/\n/g, '');
}

function replaceNewlineWithEnter() {
    novelContents.value = novelContents.value.replace(/\\n/g, '\n');
}


const getZhihuNovel = async () => {
    if (!novelUrl.value) {
        ElMessage.warning('请输入知乎小说链接');
        return;
    }

    const url = new URL(novelUrl.value);
    const mst = url.searchParams.get('mst');

    if (mst) {
        const newUrl = `https://story.zhihu.com/blogger/next-manuscript/paid_column/${mst}`;
        open(newUrl);
    }
}

const addTag = () => {
    if (tagInput.value.trim() && !tags.value.includes(tagInput.value.trim())) {
        tags.value.push(tagInput.value.trim());
        localStorage.setItem('tags', JSON.stringify(tags.value));
        tagInput.value = '';
    }
};

const removeTag = (index: number) => {
    tags.value.splice(index, 1);
    localStorage.setItem('tags', JSON.stringify(tags.value));
};

const removeLastTag = (event: KeyboardEvent) => {
    if (event.key === 'Backspace' && !tagInput.value && tags.value.length) {
        tags.value.pop();
        localStorage.setItem('tags', JSON.stringify(tags.value));
    }
};

const handleVideoListUpdate = (newList: Video[]) => {
    videoList.value = newList;
};

const handleCurrentVideoListUpdate = (newList: Video[]) => {
    currentVideoList.value = newList;
};

const selectFile = async () => {
    const file = await openDialog({
        multiple: false,
        directory: false,
    });
    if (file) {
        videoPath.value = file;
        localStorage.setItem("uploadVideoPath", file);
    }
};
const upload = async () => {

    if (!videoPath.value) {
        ElMessage.warning('请选择视频');
        return;
    }
    if (tags.value.length === 0) {
        ElMessage.warning('请输入标签');
        return;
    }
    if (!novelName.value) {
        ElMessage.warning('请输入小说名');
        return;
    }

    isUpload.value = true;

    let selectedPlatforms = [
        isBilibili.value ? 'bilibili' : null,
        isDouyin.value ? 'douyin' : null,
        isKuaishou.value ? 'kuaishou' : null,
        isBaidu.value ? 'baidu' : null,
        isDayuhao.value ? 'dayuhao' : null
    ].filter(Boolean);

    await invoke("upload_video", {
        path: videoPath.value,
        tags: tags.value,
        name: novelName.value,
        platforms: selectedPlatforms
    }).then(() => {
        ElMessage.success('上传成功');
    }).finally(() => {
        isUpload.value = false;
    });
};

const formatNovel = () => {
    novelContents.value = novelContents.value
        .replace(/[“「【]/g, '')    // 替换左引号
        .replace(/[”」】]/g, '')    // 替换右引号
        .replace(/[，,。！!？?]/g, '\n') // 替换中英文的逗号、句号、问号和感叹号为换行符
        .replace(/\n\s*\n/g, '\n'); // 清除空行
};

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
    bgmList.value = [''].concat(files);
};


const generateVideo = async () => {
    isVideoGenerating.value = true;
    aeneasPercentage.value = 0;
    formatVideoPercentage.value = 0;
    synthesizeVideoPercentage.value = 0;

    try {
        if (!novelContents.value) {
            ElMessage.warning('请输入小说内容');
            return;
        }
        if (ttsOption.value === 'azuretts') {
            await azureTtsGenerateAllAudio().catch((error) => {
                ElMessage.error(`${error as string}`);
                return;
            });
        } else {
            await edgeTtsGenerateAllAudio().catch((error) => {
                ElMessage.error(`${error as string}`);
                return;
            });
        }
        generateAudioPercentage.value = 100;
        //选择视频
        const audioDuration = await getAudioDuration(convertFileSrc(OUTPUT_PATH.value + '\\audios.wav'));
        const selected_videos = videoList.value.filter(video => video.selected);
        let totalSelectedDuration = selected_videos.reduce((acc, video) => acc + video.duration, 0);
        while (totalSelectedDuration < audioDuration) {
            const randomIndex = Math.floor(Math.random() * currentVideoList.value.length);
            const randomVideo = currentVideoList.value[randomIndex];
            selected_videos.push(randomVideo);
            totalSelectedDuration += randomVideo.duration;
        }

        // 生成字幕所需的txt文件
        formatNovel();
        let novelsTextFilePath = `${OUTPUT_PATH.value}\\text.txt`;
        let text = novelContents.value.split('').join(' ');
        await invoke('write_string_to_file', { text, filePath: novelsTextFilePath });

        // 字幕生成
        let audioPath = "/workspace/novel_output/audios.wav";
        let textPath = "/workspace/novel_output/text.txt";
        let outputPath = "/workspace/novel_output/audios.srt";
        await invoke('run_aeneas_cmd', { audioPath, textPath, outputPath });
        aeneasPercentage.value = 100

        let processedCount = 0;
        //统一视频大小
        for (const video of selected_videos) {
            const videoPath = `/workspace/novel_output/video/${video.id}.mp4`;
            const outputPath = `/workspace/novel_output/video/${video.id}_${videoOrientation.value}.mp4`;
            const path = `${OUTPUT_PATH.value}\\video\\${video.id}_${videoOrientation.value}.mp4`;
            const fileExists = await invoke<boolean>('check_file_exists', { path: path });
            if (fileExists) {
                processedCount++;
                formatVideoPercentage.value = (processedCount / selected_videos.length) * 100;
                continue;
            }
            let cmd = [
                "-y",
                "-i", videoPath,
                '-vf', videoOrientation.value === 'landscape'
                    ? 'scale=-1:1080,pad=1920:1080:(ow-iw)/2:(oh-ih)/2'
                    : 'scale=1080:-1,pad=1080:1920:(ow-iw)/2:(oh-ih)/2',
                '-preset', 'fast',
                '-r', '30', // 指定输出帧率为30fps
                '-vsync', '1', // 使用恒定帧率模式
                outputPath
            ];
            await invoke('run_ffmpeg_cmd', { cmd });
            processedCount++;
            formatVideoPercentage.value = (processedCount / selected_videos.length) * 100;
        }


        // 生成用于合成全部视频的videos.txt文件
        let videoListPath = `${OUTPUT_PATH.value}\\videos.txt`;
        let videoPaths = selected_videos.map(video => `file '/workspace/novel_output/video/${video.id}_${videoOrientation.value}.mp4'`).join('\n');
        await invoke('write_string_to_file', { text: videoPaths, filePath: videoListPath });

        let filterComplex = "";

        if (selectedBgm.value && isIncludeVideoAudio.value) {
            filterComplex = `[0:a]volume=${videoAudioVolume.value}[vid_a]; [2:a]volume=${bgmVolume.value}[bgm]; [1:a][bgm][vid_a]amix=inputs=3:duration=first[a];`;
        } else if (selectedBgm.value && !isIncludeVideoAudio.value) {
            filterComplex = `[2:a]volume=${bgmVolume.value}[bgm]; [1:a][bgm]amix=inputs=2:duration=first[a];`;
        } else if (!selectedBgm.value && isIncludeVideoAudio.value) {
            filterComplex = `[0:a]volume=${videoAudioVolume.value}[vid_a]; [1:a][vid_a]amix=inputs=2:duration=first[a];`;
        } else {
            filterComplex = `[1:a]anull[a];`;
        }

        let cmd = [
            "-y",
            "-f", "concat",
            "-safe", "0",
            "-i", "/workspace/novel_output/videos.txt",
            "-i", "/workspace/novel_output/audios.wav",
            ...(selectedBgm.value ? ["-stream_loop", "-1", "-i", `/workspace/novel_output/bgm/${getFileNameFromPath(selectedBgm.value)}`] : []),
            "-filter_complex", `
                [0:v]subtitles=/workspace/novel_output/audios.srt:force_style='FontName=ZCOOL KuaiLe,FontSize=8,Spacing=-2,PrimaryColour=&H00FFFF&,WrapStyle=0,MarginV=160,MaxWidth=300'[v];
                ${novelName.value ? `[v]drawtext=text='${novelName.value}:'x='if(lt(t,2), lerp((w-text_w)/2, 100, t/2), 100)':y='if(lt(t,2), lerp((h-text_h)/2-150, 100, t/2), 100)':fontfile=/usr/share/fonts/truetype/binfonts/ZCOOLKuaiLe-Regular.ttf:fontcolor=yellow:fontsize='if(lt(t,2), lerp(200, 100, t/2), 100)':borderw=15:bordercolor=black[v];` : ''}
                ${novelIntro.value ? `[v]drawtext=text='${novelIntro.value}:'x='(w-text_w)/2':y='if(lt(t,2), lerp((h-text_h)/2+text_h/2, h, t/2), h)':fontfile=/usr/share/fonts/truetype/binfonts/ZCOOLKuaiLe-Regular.ttf:fontcolor=yellow:fontsize=120:shadowcolor=black:borderw=10:bordercolor=black[v];` : ''}
                ${filterComplex}
            `,
            "-map", "[v]",
            "-map", "[a]",
            "-c:v", "libx264",
            "-preset", "fast",
            "-r", "30",
            "-vsync", "1",
            "-c:a", "aac",
            "-shortest",
            "/workspace/novel_output/final_video.mp4"
        ];

        await invoke('run_ffmpeg_cmd', { cmd });
        synthesizeVideoPercentage.value = 100
        videoPath.value = OUTPUT_PATH.value + '\\final_video.mp4';

        await upload()

    } catch (error) {
        ElMessage.error(`操作失败: ${error as string}`);
        return;
    } finally {
        isVideoGenerating.value = false;
    }
};

const edgeTtsGenerateAllAudio = async () => {
    isEdgeTtsGenerating.value = true;
    try {
        const speaker = localStorage.getItem('edgeTtsSpeaker');
        if (!speaker) {
            throw new Error('未设置edge说话人');
        }
        const rate = parseInt(localStorage.getItem('edgeTtsRate') ?? "0");
        const audioPath = `${OUTPUT_PATH.value}\\audios.wav`;
        const text = novelContents.value;
        await invoke('edge_tts', { audioPath, speaker, rate, text });
        ElMessage.success('生成音频成功');
        audiosSrc.value = OUTPUT_PATH.value + '\\audios.wav';
    } catch (error) {
        ElMessage.error(`生成音频失败: ${error as string}`);
        throw error;
    } finally {
        isEdgeTtsGenerating.value = false;
    }
}

const azureTtsGenerateAllAudio = async () => {
    isAzureTtsGenerating.value = true;
    try {
        const audioData = await generateCompleteAudioData(novelContents.value);
        await invoke('save_novel_audio', { audioData, audioName: 'audios.wav' });
        audiosSrc.value = OUTPUT_PATH.value + '\\audios.wav';
        ElMessage.success('生成音频成功');
    } catch (error) {
        isAzureTtsGenerating.value = false;
        throw new Error(`生成音频失败: ${error as string}`);
    } finally {
        isAzureTtsGenerating.value = false;
    }
}
</script>

<style></style>