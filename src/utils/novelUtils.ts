export interface INovel {
    content: string;
    audioSrc: string | null;
    loading: boolean;
    speaker: string | null;
    emotion: string | null;
    model: string | null;
    refAudioPath: string | null;
}

export class Novel implements INovel {
    content: string;
    audioSrc: string | null;
    loading: boolean;
    speaker: string | null;
    emotion: string | null;
    model: string | null;
    refAudioPath: string | null;

    constructor({
        content = "",
        audioSrc = null,
        loading = false,
        speaker = null,
        emotion = "平静",
        model = null,
        refAudioPath = null
    }: Partial<INovel> = {}) {
        this.content = content;
        this.audioSrc = audioSrc;
        this.loading = loading;
        this.speaker = speaker;
        this.emotion = emotion;
        this.model = model;
        this.refAudioPath = refAudioPath;
    }
}
