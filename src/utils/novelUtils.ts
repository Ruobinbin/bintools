//格式话小说文本
function formatNovelText(text: string): string {
    const regex = new RegExp(`([。])([”】]?)`, 'g');
    const newText = text.replace(regex, (match) => {
        return match + '\n';
    });
    return newText;
}

interface INovel {
    content: string;
    audioSrc: string | null;
    loading: boolean;
}

class Novel implements INovel {
    constructor(
        public content: string = '',
        public audioSrc: string | null = null,
        public loading: boolean = false
    ) { }
}
export { Novel, formatNovelText };
export type { INovel };
