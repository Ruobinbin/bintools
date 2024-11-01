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
    channelUrl: string;
    duration: number;
    thumbnail: string;
    selected: boolean;
    isDownloading: boolean;
}

class Video implements IVideo {
    public id: string;
    public url: string;
    public channelUrl: string;
    public duration: number;
    public thumbnail: string;
    public selected: boolean;
    public isDownloading: boolean;

    constructor(id: string, url: string, channelUrl: string, duration: number, thumbnail: string) {
        this.id = id;
        this.url = url;
        this.channelUrl = channelUrl;
        this.duration = duration;
        this.thumbnail = thumbnail;
        this.selected = false;
        this.isDownloading = false;
    }
}
export { Video };
export type { IVideo, IThumbnail };
