// utils.ts

// ISO 8601 转 秒
export const ISO8601ToSeconds = (duration: string): number => {
    const match = duration.match(/PT(\d+H)?(\d+M)?(\d+S)?/);
    if (match) {
        const hours = parseInt(match[1]) || 0;
        const minutes = parseInt(match[2]) || 0;
        const seconds = parseInt(match[3]) || 0;

        return hours * 3600 + minutes * 60 + seconds;
    } else {
        return 0;
    }
};

// 去除 HTML 标签
export const stripHtmlTags = (html: string): string => {
    const div = document.createElement("div");
    div.innerHTML = html;
    return div.textContent || div.innerText || "";
};

// 获取文件名
export const getFileNameFromPath = (path: string): string => {
    const match = path.match(/[^/\\]+$/);
    return match ? match[0] : '';
};

export const getFileNameFromPathWithoutExtension = (path: string): string => {
    const baseName = getFileNameFromPath(path);
    return baseName.substring(0, baseName.lastIndexOf('.'));
};

// 将秒数转换为时分秒格式
export const secondsToHMS = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    const parts = [
        hours > 0 ? `${hours}小时` : '',
        minutes > 0 ? `${minutes}分钟` : '',
        `${secs}秒`
    ];

    return parts.filter(Boolean).join('');
};

// 获取音频时长
export const getAudioDuration = (path: string): Promise<number> => {
    return new Promise((resolve, reject) => {
        const audio = new Audio(path);
        audio.addEventListener('loadedmetadata', () => {
            resolve(audio.duration);
        });
        audio.addEventListener('error', () => {
            reject(new Error('获取音频时长失败'));
        });
    });
};