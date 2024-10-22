import { invoke } from "@tauri-apps/api/core";
import { insertVideo } from "./dbUtils";

interface IThumbnail {
    url: string;
    height: number;
    width: number;
}

interface IVideo {
    id: string;
    url: string;
    duration: number;
    thumbnails: IThumbnail[];
    selected: boolean;
    isDownloading: boolean; // 新增变量
}

class Video implements IVideo {
    public id: string;
    public url: string;
    public duration: number;
    public thumbnails: IThumbnail[];
    public selected: boolean;
    public isDownloading: boolean;

    constructor(id: string, url: string, duration: number | string, thumbnails: IThumbnail[]) {
        this.id = id;
        this.url = url;
        this.duration = this.validateDuration(duration);
        this.thumbnails = thumbnails;
        this.selected = false;
        this.isDownloading = false;
    }

    public validateDuration(value: number | string): number {
        const num = typeof value === 'number' ? value : Number(value);
        return !isNaN(num) ? num : -1;
    }

    public getLargestThumbnailUrl(): string {
        if (this.thumbnails.length === 0) {
            return 'https://via.placeholder.com/600x400.png?text=No+Image';
        }

        const largestThumbnail = this.thumbnails.reduce((largest, current) => {
            const largestArea = largest.height * largest.width;
            const currentArea = current.height * current.width;
            return currentArea > largestArea ? current : largest;
        });

        return largestThumbnail.url;
    }

    public async downloadVideo(path: string): Promise<void> {
        this.isDownloading = true;
        const outputFilePath = `${path}/${this.id}.mp4`;

        const cmd = [
            this.url,
            '-f', 'bestvideo[ext=mp4]+bestaudio[ext=m4a]/mp4', // 选择最佳的 mp4 格式
            '-o', outputFilePath,
            '--merge-output-format', 'mp4' // 确保合并后的格式为 mp4
        ];

        await invoke<string>('run_yt_dlp_cmd', { cmd })
            .then(logs => {
                if (logs.includes('100% of')) {
                    console.log(logs);
                    return insertVideo(this);
                } else {
                    throw new Error('下载失败');
                }
            })
            .catch(error => {
                console.error(error);
            })
            .finally(() => {
                this.isDownloading = false;
            });
    }
}
export { Video };
export type { IVideo, IThumbnail };
