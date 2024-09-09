use crate::utils;
use std::path::Path;
use std::path::PathBuf;
use tauri::command;

use crate::{gpt_sovits_model_dir, novel_output_dir};

//测试命令
#[command]
pub fn input_enter(value: &str) -> String {
    println!("input_enter: {}", value);
    value.to_string()
}

//判断容器是否运行
#[command]
pub async fn is_container_running(container_name: &str) -> Result<bool, String> {
    utils::bollard_utils::is_container_running(container_name)
        .await
        .map_err(|e| e.to_string())
}

//启动gpt-sovits-api
#[command]
pub async fn start_gpt_sovits_api() -> Result<(), String> {
    utils::bollard_utils::start_gpt_sovits_api()
        .await
        .map_err(|e| e.to_string())
}

//保存小说音频
#[command]
pub async fn save_novel_audio(audio_data: Vec<u8>, audio_name: &str) -> Result<String, String> {
    let file_path = novel_output_dir().join(audio_name);
    match utils::default_utils::write_audio_to_file(audio_data, file_path.clone()) {
        Ok(_) => Ok(file_path.to_string_lossy().to_string()),
        Err(e) => Err(e.to_string()),
    }
}

//打开路径
#[command]
pub async fn open_path(path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(path);
    utils::default_utils::open_path(path_buf).map_err(|e| e.to_string())
}

//获取gpt_sovits模型列表
#[command]
pub async fn get_gpt_sovits_models() -> Result<Vec<utils::gpt_sovits_utils::GptSovitsModel>, String>
{
    let path = gpt_sovits_model_dir();
    let models = utils::gpt_sovits_utils::get_gpt_sovits_models(&path);
    Ok(models)
}

//写入字符串到文件
#[command]
pub fn write_string_to_file(text: &str, file_path: String) -> Result<(), String> {
    let file_path = PathBuf::from(file_path);
    let _ = utils::default_utils::write_string_to_file(text, file_path).map_err(|e| e.to_string());
    Ok(())
}

//运行ffmpeg命令
#[command]
pub async fn run_ffmpeg_cmd(cmd: Vec<&str>) -> Result<(), String> {
    utils::bollard_utils::create_and_run_ffmpeg_container(cmd)
        .await
        .map_err(|e| e.to_string())
}

//运行aeneas命令
#[command]
pub async fn run_aeneas_cmd(
    audio_path: String,
    text_path: String,
    output_path: String,
) -> Result<(), String> {
    utils::bollard_utils::create_and_run_aeneas_container(audio_path, text_path, output_path)
        .await
        .map_err(|e| e.to_string())
}

//运行yt_dlp命令
#[command]
pub async fn run_yt_dlp_cmd(cmd: Vec<&str>) -> Result<String, String> {
    match utils::bollard_utils::create_and_run_yt_dlp_container(cmd).await {
        Ok(logs) => Ok(logs),
        Err(e) => Err(e.to_string()),
    }
}

//检查文件是否存在
#[command]
pub fn check_file_exists(path: String) -> bool {
    Path::new(&path).exists()
}

//创建文件夹并获取文件夹下的文件列表
#[command]
pub fn create_dir_and_get_files(path: String) -> Vec<String> {
    let dir_path = Path::new(&path);

    utils::default_utils::ensure_path_exists(&dir_path.to_path_buf()).unwrap();

    let files = utils::default_utils::get_files_in_dir(dir_path);
    files
        .into_iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect()
}

//打开路径或文件
#[command]
pub async fn open_path_or_file(path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    match utils::default_utils::open_path_or_file(path_buf) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
