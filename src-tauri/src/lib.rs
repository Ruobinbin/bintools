mod tauri_cmd;
pub mod utils;

use once_cell::sync::OnceCell;
use std::path::PathBuf;

use tauri::{
    // menu::{MenuBuilder, MenuItemBuilder},
    // tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle,
    Manager,
};

pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new(); //APP句柄
pub static APP_RESOURCE_DIR: OnceCell<PathBuf> = OnceCell::new(); //app所在目录
pub static USER_FILES_DIR: OnceCell<PathBuf> = OnceCell::new(); //用户文件所在目录
pub static GPT_SOVITS_MODEL_DIR: OnceCell<PathBuf> = OnceCell::new(); //gpt-sovits模型所在目录
pub static NOVEL_OUTPUT_DIR: OnceCell<PathBuf> = OnceCell::new(); //小说输出目录

pub fn app_handle() -> AppHandle {
    APP_HANDLE.get().unwrap().clone()
}

pub fn app_resource_dir() -> PathBuf {
    APP_RESOURCE_DIR.get().unwrap().clone()
}

pub fn user_files_dir() -> PathBuf {
    USER_FILES_DIR.get().unwrap().clone()
}

pub fn gpt_sovits_model_dir() -> PathBuf {
    GPT_SOVITS_MODEL_DIR.get().unwrap().clone()
}

pub fn novel_output_dir() -> PathBuf {
    NOVEL_OUTPUT_DIR.get().unwrap().clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 初始化APP句柄
            APP_HANDLE.set(app.handle().clone()).unwrap();
            // 初始化数据目录
            let resources_dir = app.path().resource_dir().unwrap();
            APP_RESOURCE_DIR
                .set(utils::default_utils::remove_long_path_prefix(
                    &resources_dir,
                ))
                .unwrap(); //app所在目录
            USER_FILES_DIR
                .set(APP_RESOURCE_DIR.get().unwrap().join("user_files"))
                .unwrap(); //用户文件所在目录
            GPT_SOVITS_MODEL_DIR
                .set(USER_FILES_DIR.get().unwrap().join("gpt_sovits_model"))
                .unwrap(); //gpt-sovits模型所在目录
            NOVEL_OUTPUT_DIR
                .set(USER_FILES_DIR.get().unwrap().join("novel_output"))
                .unwrap();
            utils::default_utils::ensure_path_exists(USER_FILES_DIR.get().unwrap()).unwrap();
            utils::default_utils::ensure_path_exists(GPT_SOVITS_MODEL_DIR.get().unwrap()).unwrap();
            utils::default_utils::ensure_path_exists(NOVEL_OUTPUT_DIR.get().unwrap()).unwrap();
            //系统托盘
            // let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            // let menu = MenuBuilder::new(app).items(&[&quit]).build()?;
            // let _tray = TrayIconBuilder::new()
            //     .menu(&menu)
            //     .on_menu_event(move |app, event| match event.id().as_ref() {
            //         "quit" => app.exit(0),
            //         _ => (),
            //     })
            //     .on_tray_icon_event(|tray, event| {
            //         if let TrayIconEvent::Click {
            //             button: MouseButton::Left,
            //             button_state: MouseButtonState::Up,
            //             ..
            //         } = event
            //         {
            //             let app = tray.app_handle();
            //             if let Some(webview_window) = app.get_webview_window("main") {
            //                 let _ = webview_window.show();
            //                 let _ = webview_window.set_focus();
            //             }
            //         }
            //     })
            //     .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build()) // 全局快捷键
        .plugin(tauri_plugin_sql::Builder::default().build()) // SQLite数据库
        .plugin(tauri_plugin_http::init()) // HTTP请求
        .invoke_handler(tauri::generate_handler![
            tauri_cmd::input_enter,
            tauri_cmd::is_container_running,
            tauri_cmd::start_gpt_sovits_api,
            tauri_cmd::save_novel_audio,
            tauri_cmd::open_path,
            tauri_cmd::get_gpt_sovits_models,
            tauri_cmd::write_string_to_file,
            tauri_cmd::run_ffmpeg_cmd,
            tauri_cmd::run_aeneas_cmd,
            tauri_cmd::run_yt_dlp_cmd,
            tauri_cmd::check_file_exists,
            tauri_cmd::create_dir_and_get_files,
            tauri_cmd::open_path_or_file,
            tauri_cmd::edge_tts,
            tauri_cmd::edge_tts_get_voices,
            tauri_cmd::delete_file,
            tauri_cmd::delete_file_with_prefix,
            tauri_cmd::upload_video,
            tauri_cmd::proxy_request,
            tauri_cmd::decompress_brotli,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
