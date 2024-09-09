import Database from '@tauri-apps/plugin-sql';
import { INovel } from './novelUtils';
import { IThumbnail, IVideo } from './ytdlpUitls';

// 缓存数据库实例
let dbInstance: Database | null = null;

// 获取数据库实例
export const getDatabase = async (): Promise<Database> => {
    if (!dbInstance) {
        dbInstance = await Database.load('sqlite:bintool.db');
    }
    return dbInstance;
};

// 获取所有表的名称
export const getAllTableNames = async (): Promise<string[]> => {
    const db = await getDatabase();
    const result = await db.select("SELECT name FROM sqlite_master WHERE type='table';") as { name: string }[];
    return result.map(row => row.name);
};

// 根据表名获取数据
export const getTableDataByName = async (tableName: string): Promise<Array<Record<string, any>>> => {
    const db = await getDatabase();
    const result = await db.select(`SELECT * FROM ${tableName};`);
    return result as Array<Record<string, any>>;
};

// 创建小说表格
const createNovelsTable = async () => {
    const db = await getDatabase();
    await db.execute(`
        CREATE TABLE IF NOT EXISTS novels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            audioSrc TEXT NOT NULL,
            content TEXT NOT NULL
        )
    `);
};

// 重置小说表格
export const resetNovelsTable = async (novels: INovel[]): Promise<void> => {
    const db = await getDatabase();
    await createNovelsTable();
    await db.execute('DELETE FROM novels');
    for (const novel of novels) {
        await db.execute('INSERT INTO novels (content, audioSrc) VALUES (?, ?)', [novel.content, novel.audioSrc]);
    }
};

// 获取所有小说
export const getAllNovels = async (): Promise<INovel[]> => {
    try {
        const db = await getDatabase();
        const result = await db.select('SELECT * FROM novels') as INovel[]
        return result;
    } catch (e) {
        return [];
    }
};
//=====================================

// 创建视频表格
const createVideosTable = async () => {
    const db = await getDatabase();
    await db.execute(`
        CREATE TABLE IF NOT EXISTS videos (
            id TEXT PRIMARY KEY,
            url TEXT,
            duration INTEGER,
            thumbnails TEXT
        )
    `);
};

export const insertVideo = async (video: IVideo): Promise<void> => {
    const db = await getDatabase();
    await createVideosTable();
    await db.execute(
        `INSERT INTO videos (id, url, duration, thumbnails) VALUES (?, ?, ?, ?)`,
        [video.id, video.url, video.duration, JSON.stringify(video.thumbnails)]
    );
};

// 获取所有视频数据
export const getAllVideos = async (): Promise<IVideo[]> => {
    try {
        const db = await getDatabase();
        const result = await db.select('SELECT * FROM videos') as Array<{
            id: string;
            url: string;
            duration: number;
            thumbnails: string;
        }>;

        return result.map(row => ({
            id: row.id,
            url: row.url,
            duration: row.duration,
            thumbnails: JSON.parse(row.thumbnails) as IThumbnail[],
            selected: false,
        }));
    } catch (error) {
        return [];
    }
};