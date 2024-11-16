<template>
    <el-text>所选视频时长: {{ totalVideoDuration }}</el-text>
    <div flex items-center>
        <el-select v-model="selectedChannel" placeholder="选择博主主页链接">
            <el-option v-for="channel in channelUrls" :key="channel" :label="channel" :value="channel">
                {{ channel }}
            </el-option>
        </el-select>
        <el-button type="default" @click="open(selectedChannel)">打开</el-button>
        <div w-20>
            <el-input v-model="fetchVideoContent" placeholder="获取数量" />
        </div>
        <el-button :loading="isGetting" type="success" @click="getVideoList">获取</el-button>
        <el-button type="primary" @click="addNewChannelUrl">添加</el-button>
        <el-button type="danger" @click="deleteCurrentChannelUrl">删除</el-button>
    </div>
    <div>
        <el-collapse v-model="activeCollapse" bg-black>
            <el-collapse-item title="本地视频" name="1">
                <div flex flex-wrap gap-2>
                    <div :class="{ 'bg-yellow': video.selected, 'bg-gray': !video.selected }" flex-0-1-auto w-50
                        rounded-lg v-for="video in currentVideoList" :key="video.id">
                        <img :src="video.thumbnail" :alt="video.url" @click="toggleVideoSelection(video)" w-full h-auto
                            rounded-lg />
                        <p><el-link type="primary" @click.prevent="open(video.url)">点击此处观看</el-link></p>
                        <p><el-text>ID: {{ video.id }}</el-text></p>
                        <p><el-text>时长: {{ video.duration }} 秒</el-text></p>
                        <el-button @click="delVideo(video.id)">删除</el-button>
                        <el-button @click="downloadVideo(video)" :loading="video.isDownloading">下载</el-button>
                    </div>
                </div>
            </el-collapse-item>
            <el-collapse-item title="网络视频" name="2">
                <div flex flex-wrap gap-2>
                    <div :class="{ 'bg-yellow': video.selected, 'bg-gray': !video.selected }" flex-0-1-auto w-50
                        rounded-lg v-for="video in newVideoList" :key="video.id">
                        <img :src="video.thumbnail" :alt="video.url" @click="addTOVideoList(video)" w-full h-auto
                            rounded-lg />
                        <p><el-link type="primary" @click.prevent="open(video.url)">点击此处观看</el-link></p>
                        <p><el-text>ID: {{ video.id }}</el-text></p>
                        <p><el-text>时长: {{ video.duration }} 秒</el-text></p>
                        <el-button @click="delVideo(video.id)">删除</el-button>
                        <el-button @click="downloadVideo(video)" :loading="video.isDownloading">下载</el-button>
                    </div>
                </div>
            </el-collapse-item>
        </el-collapse>
    </div>
    <div>
    </div>
</template>
<script lang="ts" setup>
// ''
import { open } from '@tauri-apps/plugin-shell';
import { invoke } from '@tauri-apps/api/core';
import { IThumbnail, Video } from '../../utils/ytdlpUitls'
import { getAllVideos, getAllChannelUrls, addChannelUrl, deleteChannelUrlByUrl, deleteVideoById, insertVideo } from '../../utils/dbUtils'
import { ElButton, ElMessage, ElMessageBox } from 'element-plus';
import { computed, onMounted, ref, watch } from 'vue';
import { resourceDir } from '@tauri-apps/api/path';

let videoList = ref<Video[]>([]);//视频列表
let newVideoList = ref<Video[]>([]);//新视频列表
let totalVideoDuration = computed(() => {
    return videoList.value
        .filter(video => video.selected)
        .reduce((total, video) => total + video.duration, 0);
});//视频总时长
let channelUrls = ref<string[]>([]); // 博主主页链接列表
let selectedChannel = ref(''); // 当前选中的博主主页链接
let isGetting = ref(false); // 是否正在获取
const VIDEO_PATH = ref('');//输出路径
let activeCollapse = ref('1'); // 默认展开第一个
let fetchVideoContent = ref(50)
let currentVideoList = computed(() => videoList.value.filter(video => video.channelUrl === selectedChannel.value));//当前视频列表

const emit = defineEmits(['updateVideoList', 'updateCurrentVideoList'])

// 监听 videoList 的变化
watch(videoList, (newList) => {
    emit('updateVideoList', newList);
});

watch(currentVideoList, (newList) => {
    emit('updateCurrentVideoList', newList);
});

// 监听 selectedChannel 的变化
watch(selectedChannel, (newChannel) => {
    localStorage.setItem('channelUrl', newChannel);
});


//载入时触发
onMounted(async () => {
    selectedChannel.value = localStorage.getItem('channelUrl') || '';
    VIDEO_PATH.value = (await resourceDir()) + '\\user_files\\novel_output\\video';
    videoList.value = (await getAllVideos()).map(video => {
        return new Video(video.id, video.url, video.channelUrl, video.duration, video.thumbnail);
    });
    console.log(videoList.value);
    channelUrls.value = await getAllChannelUrls();
});

const downloadVideo = async (video: Video): Promise<void> => {
    if (videoList.value.some(v => v.id === video.id)) {
        ElMessage.error("本地已存在此视频")
        return;
    }

    video.isDownloading = true;
    const outputFilePath = `/workspace/novel_output/video/${video.id}.mp4`;

    const cmd = [
        video.url,
        '-f', 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/mp4',
        '-o', outputFilePath,
        '--merge-output-format', 'mp4'
    ];

    await invoke<string>('run_yt_dlp_cmd', { cmd })
        .then(logs => {
            if (logs.includes('100% of')) {
                insertVideo(video);
                videoList.value.push(video);
                return;
            } else {
                throw new Error('下载失败');
            }
        })
        .catch(error => {
            throw new Error(`下载视频失败: ${error}`);
        })
        .finally(() => {
            video.isDownloading = false;
        });
}

const addTOVideoList = (video: Video) => {
    if (videoList.value.some(v => v.id === video.id)) {
        ElMessage.error("本地已存在此视频")
        return;
    }
    video.selected = true
    videoList.value.push(video);
    ElMessage.success("成功添加到本地")
}

const toggleVideoSelection = (video: Video) => {
    video.selected = !video.selected;
    emit('updateVideoList', videoList.value);
};

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

//删除视频
const delVideo = async (id: string) => {
    try {
        const dirPath = VIDEO_PATH.value
        const prefix = id
        await invoke<boolean>('delete_file_with_prefix', { dirPath, prefix })
        await deleteVideoById(id);
        videoList.value = videoList.value.filter(video => video.id !== id);
        ElMessage.success(`删除视频成功`);
    } catch (error) {
        ElMessage.error(`删除视频失败: ${error as string}`);
    }
}

// 删除当前选中的博主URL
const deleteCurrentChannelUrl = async () => {
    if (selectedChannel.value) {

        let videos = videoList.value.filter(video => video.channelUrl === selectedChannel.value);
        newVideoList.value = [];
        for (const video of videos) {
            await delVideo(video.id);
        }

        await deleteChannelUrlByUrl(selectedChannel.value);
        channelUrls.value = channelUrls.value.filter(channel => channel !== selectedChannel.value);
        selectedChannel.value = '';
        ElMessage.success('删除博主主页链接成功');
    } else {
        ElMessage.warning('请选择一个博主链接');
    }
}


//获取视频列表
const getVideoList = async () => {
    newVideoList.value = [];
    isGetting.value = true;
    const cmd = [
        '--flat-playlist',
        '--print-json',
        '--playlist-end', `${fetchVideoContent.value}`,
        selectedChannel.value
    ];

    await invoke('run_yt_dlp_cmd', { cmd }).then((log) => {
        let logStr = log as string;
        logStr.trim().split('\n').forEach(videoStr => {
            try {
                let video = JSON.parse(videoStr);
                let thumbnails: IThumbnail[] = video.thumbnails;
                let thumbnail = '';

                if (thumbnails.length === 0) {
                    thumbnail = 'https://via.placeholder.com/600x400.png?text=No+Image';
                } else {
                    let largestThumbnail = thumbnails.reduce((largest, current) => {
                        const largestArea = largest.height * largest.width;
                        const currentArea = current.height * current.width;
                        return currentArea > largestArea ? current : largest;
                    });
                    thumbnail = largestThumbnail.url;
                }

                const num = typeof video.duration === 'number' ? video.duration : Number(video.duration);
                const duration = !isNaN(num) ? num : -1;

                newVideoList.value.push(new Video(video.id, video.url, selectedChannel.value, duration, thumbnail));
            } catch (error) {
                // 如果解析失败，不会push到videoList.value中
            }
        });
        ElMessage.success('博主视频获取成功');
    }).catch((error) => {
        ElMessage.error(`博主视频获取失败: ${error as string}`);
    }).finally(() => {
        isGetting.value = false;
    });
}
</script>

<style></style>