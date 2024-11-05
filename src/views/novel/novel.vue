<template>
    <el-tabs tab-position="left">
        <el-tab-pane label="小说">
            <el-text class="mx-1" type="info">长度:{{ novelContents.length }}</el-text>
            <div style="display: flex; align-items: center;">
                <el-input v-model="novelUrl" placeholder="小说链接" />
                <el-button @click="getZhihuNovel">打开知乎小说</el-button>
            </div>
            <el-button @click="removeNumberedLines">去除编号</el-button>
            <el-button @click="edgeTtsGenerateAllAudio" :loading="isEdgeTtsGenerating">edgeTts生成音频</el-button>
            <el-button @click="azureTtsGenerateAllAudio" :loading="isAzureTtsGenerating">Azure TTS生成音频</el-button>
            <audio :src="`${convertFileSrc(audiosSrc)}?t=${new Date().getTime()}`" controls></audio>
            <el-input v-model="novelContents" style="width: 100%;" autosize type="textarea" placeholder="小说内容" />
        </el-tab-pane>
        <el-tab-pane label="视频">
            <VideoList @updateVideoList="handleVideoListUpdate"
                @updateCurrentVideoList="handleCurrentVideoListUpdate" />
        </el-tab-pane>
        <el-tab-pane label=" 最后合成">
            <div>
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
            <el-button @click="generateVideo">合成视频</el-button>
            <el-button @click="open(OUTPUT_PATH)">打开输出目录</el-button>
            <el-divider>设置</el-divider>
            <el-input v-model="novelName" type="textarea" placeholder="小说名" />
            <el-input v-model="novelIntro" type="textarea" placeholder="小说介绍" />
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
                <el-input v-model="BgmUrl" placeholder="下载BGM" @keyup.enter="downloadBgm(BgmUrl)"></el-input>
                <audio v-if="selectedBgm" :src="convertFileSrc(selectedBgm)" controls></audio>
                <el-slider v-if="selectedBgm" v-model="bgmVolume" :min="0" :max="1" :step="0.01" show-stops
                    inline></el-slider>
            </div>
            <el-divider>上传</el-divider>
            <div>
                <div class="tags">
                    <el-tag v-for="(tag, index) in tags" :key="index" closable @close="removeTag(index)"
                        style="margin-right: 5px;">
                        {{ tag }}
                    </el-tag>
                </div>
                <el-input v-model="tagInput" placeholder="输入标签并按回车" @keyup.enter="addTag"
                    @keydown.backspace="removeLastTag">
                </el-input>
            </div>
            <video width="10%" :src="`${convertFileSrc(videoPath)}?t=${new Date().getTime()}`" controls />
            <el-button @click="selectFile">选择文件</el-button>
            <el-input v-model="videoPath" placeholder="视频路径" readonly></el-input>
            <el-button @click="upload('bilibili')" :loading="isBilibiliUpload">bilibili上传</el-button>
            <el-button @click="upload('douyin')" :loading="isDyUpload">抖音上传</el-button>
            <el-button @click="upload('ks')" :loading="isKsUpload">快手上传</el-button>
            <el-button @click="upload('wx')" :loading="isWxUpload">微信上传</el-button>
            <el-button @click="upload('bd')" :loading="isBdUpload">百度上传</el-button>
            <el-button @click="upload('all')" :loading="isAllUpload">一键上传</el-button>
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
let videoAudioVolume = ref(0.1); // 视频音频音量
let novelName = ref(''); // 小说名
let novelIntro = ref(''); // 小说介绍
let aeneasPercentage = ref(0)
let formatVideoPercentage = ref(0); // 统一格式进度
let synthesizeVideoPercentage = ref(0); // 合成视频进度
let videoPath = ref("");
let isBilibiliUpload = ref(false);
let isDyUpload = ref(false);
let isKsUpload = ref(false);
let isWxUpload = ref(false);
let isBdUpload = ref(false);
let isAllUpload = ref(false);
let tagInput = ref('');
let tags = ref<string[]>([]);
let novelUrl = ref('');
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

function removeNumberedLines() {
    novelContents.value = novelContents.value.replace(/^\d+\s*$/gm, '');
    novelContents.value = novelContents.value.replace(/^\s*[\r\n]/gm, '');
    novelContents.value = novelContents.value.replace(/\n/g, '');
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
const upload = async (platform: string) => {

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

    if (platform === 'bilibili') {
        isBilibiliUpload.value = true;
    } else if (platform === 'douyin') {
        isDyUpload.value = true;
    } else if (platform === 'ks') {
        isKsUpload.value = true;
    } else if (platform === 'wx') {
        isWxUpload.value = true;
    } else if (platform === 'bd') {
        isBdUpload.value = true;
    } else if (platform === 'all') {
        isAllUpload.value = true;
    }

    await invoke("upload_video", { platform, path: videoPath.value, tags: tags.value, name: novelName.value }).then(() => {
        ElMessage.success('上传成功');
    }).finally(() => {
        isBilibiliUpload.value = false;
        isDyUpload.value = false;
        isKsUpload.value = false;
        isWxUpload.value = false;
        isBdUpload.value = false;
        isAllUpload.value = false;
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
    aeneasPercentage.value = 0;
    formatVideoPercentage.value = 0;
    synthesizeVideoPercentage.value = 0;

    try {
        // 检查小说内容
        if (!novelContents.value) {
            ElMessage.warning('请输入小说内容');
            return;
        }

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
                [0:v]subtitles=/workspace/novel_output/audios.srt:force_style='FontName=ZCOOL KuaiLe,FontSize=8,Spacing=-2,PrimaryColour=&H00FFFF&,WrapStyle=0,MarginV=160,Width=10'[v];
                ${novelName.value ? `[v]drawtext=text='${novelName.value}:'x='if(lt(t,2), lerp((w-text_w)/2, 50, t/2), 50)':y='if(lt(t,2), lerp((h-text_h)/2-150, 50, t/2), 50)':fontfile=/usr/share/fonts/truetype/binfonts/ZCOOLKuaiLe-Regular.ttf:fontcolor=yellow:fontsize='if(lt(t,2), lerp(300, 100, t/2), 100)':shadowcolor=black:shadowx=10:shadowy=10[v];` : ''}
                ${novelIntro.value ? `[v]drawtext=text='${novelIntro.value}:'x='(w-text_w)/2':y='if(lt(t,2), lerp((h-text_h)/2+150+text_h/2-150, h, t/2), h)':fontfile=/usr/share/fonts/truetype/binfonts/ZCOOLKuaiLe-Regular.ttf:fontcolor=yellow:fontsize=150:shadowcolor=black:shadowx=10:shadowy=10[v];` : ''}
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

    } catch (error) {
        ElMessage.error(`操作失败: ${error as string}`);
        return;
    }
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

const azureTtsGenerateAllAudio = async () => {
    isAzureTtsGenerating.value = true;
    await generateCompleteAudioData(novelContents.value,)
        .then(async (audioData) => {
            await invoke('save_novel_audio', { audioData, audioName: 'audios.wav' }).then(() => {
                audiosSrc.value = OUTPUT_PATH.value + '\\audios.wav';
                ElMessage.success('生成音频成功');
            }).catch(error => {
                ElMessage.error(`保存音频失败: ${error as string}`);
            });
        })
        .catch(error => {
            ElMessage.error(`生成音频失败: ${error as string}`);
        })
        .finally(() => {
            isAzureTtsGenerating.value = false;
        });
}
</script>

<style></style>