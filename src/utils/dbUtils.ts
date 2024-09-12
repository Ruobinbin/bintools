import Database from '@tauri-apps/plugin-sql';
import { INovel } from './novelUtils';
import { IThumbnail, IVideo } from './ytdlpUitls';

// 缓存数据库实例
let db = await Database.load('sqlite:bintool.db');

//====================基础====================

// 获取所有表的名称
export const getAllTableNames = async (): Promise<string[]> => {
    const result = await db.select("SELECT name FROM sqlite_master WHERE type='table';") as { name: string }[];
    return result.map(row => row.name);
};

// 根据表名获取数据
export const getTableDataByName = async (tableName: string): Promise<Array<Record<string, any>>> => {
    const result = await db.select(`SELECT * FROM ${tableName};`);
    return result as Array<Record<string, any>>;
};

// 根据表名删除表
export const deleteTableByName = async (tableName: string): Promise<void> => {
    await db.execute(`DROP TABLE IF EXISTS ${tableName}`);
};

//====================初始化====================
export const dbInit = async (): Promise<void> => {
    // 创建小说表格
    await db.execute(`
        CREATE TABLE IF NOT EXISTS novels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            audioSrc TEXT NOT NULL,
            content TEXT NOT NULL
        )
    `);

    // 创建视频表格
    await db.execute(`
        CREATE TABLE IF NOT EXISTS videos (
            id TEXT PRIMARY KEY,
            url TEXT,
            duration INTEGER,
            thumbnails TEXT
        )
    `);

    // 创建博主主页URL表格
    await db.execute(`
        CREATE TABLE IF NOT EXISTS channel_urls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT UNIQUE NOT NULL
        )
    `);

    // 创建设置表
    await db.execute(`
        CREATE TABLE IF NOT EXISTS settings (
            key_name TEXT PRIMARY KEY,
            value TEXT
        )
    `);

    // 创建大语言模型API表
    await db.execute(`
        CREATE TABLE IF NOT EXISTS llm_api (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            api_key TEXT NOT NULL
        )
    `);
};

//====================小说====================
// 重置小说表格
export const resetNovelsTable = async (novels: INovel[]): Promise<void> => {
    await db.execute('DELETE FROM novels');
    for (const novel of novels) {
        await db.execute('INSERT INTO novels (content, audioSrc) VALUES (?, ?)', [novel.content, novel.audioSrc]);
    }
};

// 获取所有小说
export const getAllNovels = async (): Promise<INovel[]> => {
    const result = await db.select('SELECT * FROM novels') as INovel[];
    return result;
};

//====================视频====================
// 插入视频
export const insertVideo = async (video: IVideo): Promise<void> => {
    await db.execute(
        `INSERT INTO videos (id, url, duration, thumbnails) VALUES (?, ?, ?, ?)`,
        [video.id, video.url, video.duration, JSON.stringify(video.thumbnails)]
    );
};

// 获取所有视频数据
export const getAllVideos = async (): Promise<IVideo[]> => {
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
};

// 获取所有博主的URL
export const getAllChannelUrls = async (): Promise<string[]> => {
    const result = await db.select('SELECT url FROM channel_urls') as { url: string }[];
    return result.map(row => row.url);
};

// 添加博主URL
export const addChannelUrl = async (url: string): Promise<boolean> => {
    await db.execute('INSERT INTO channel_urls (url) VALUES (?)', [url]);
    return true;
};

// 根据id删除博主URL
export const deleteChannelUrlById = async (id: number): Promise<boolean> => {
    await db.execute('DELETE FROM channel_urls WHERE id = ?', [id]);
    return true;
};

// 根据URL删除博主URL
export const deleteChannelUrlByUrl = async (url: string): Promise<boolean> => {
    await db.execute('DELETE FROM channel_urls WHERE url = ?', [url]);
    return true;
};

//====================设置====================
// 获取设置值
export const getSetting = async (keyName: string): Promise<string | null> => {
    const result = await db.select<{ value: string }[]>('SELECT value FROM settings WHERE key_name = ?', [keyName]);
    return result.length > 0 ? result[0].value : null;
};

// 更新设置值
export const setSetting = async (keyName: string, value: string): Promise<boolean> => {
    await db.execute('INSERT OR REPLACE INTO settings (key_name, value) VALUES (?, ?)', [keyName, value]);
    return true;
};

//====================聊天API====================
// 添加新的API
export const addLLMApi = async (name: string, url: string, apiKey: string): Promise<boolean> => {
    await db.execute('INSERT INTO llm_api (name, url, api_key) VALUES (?, ?, ?)', [name, url, apiKey]);
    return true;
};

// 获取所有API
export const getAllLLMApis = async (): Promise<Array<{ id: number, name: string, url: string, api_key: string }>> => {
    const result = await db.select('SELECT * FROM llm_api');
    return result as Array<{ id: number, name: string, url: string, api_key: string }>;
};

// 更新API
export const updateLLMApi = async (id: number, name: string, url: string, apiKey: string): Promise<boolean> => {
    await db.execute('UPDATE llm_api SET name = ?, url = ?, api_key = ? WHERE id = ?', [name, url, apiKey, id]);
    return true;
};

// 删除API
export const deleteLLMApi = async (id: number): Promise<boolean> => {
    await db.execute('DELETE FROM llm_api WHERE id = ?', [id]);
    return true;
};
