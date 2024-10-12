use msedge_tts::{tts::client::connect, tts::SpeechConfig, voice::get_voices_list};
use std::fs::File;
use std::io::Write;
pub fn edge_tts(speaker: &str, audio_path: &str, rate: i32, text: &str) -> Result<(), String> {
    let voices = get_voices_list().map_err(|e| format!("获取声音列表失败: {:?}", e))?;
    for voice in &voices {
        if voice.name == speaker {
            let mut config = SpeechConfig::from(voice);
            config.rate = rate;
            let mut tts = connect().map_err(|e| format!("连接TTS服务失败: {:?}", e))?;
            let audio = tts
                .synthesize(text, &config)
                .map_err(|e| format!("合成音频失败: {:?}", e))?;
            let mut file =
                File::create(audio_path).map_err(|e| format!("创建文件失败: {:?}", e))?;
            file.write_all(&audio.audio_bytes)
                .map_err(|e| format!("写入音频数据失败: {:?}", e))?;
            return Ok(());
        }
    }
    Err("未找到指定的说话人".to_string())
}

pub fn edge_tts_get_voices() -> Vec<String> {
    let voices = get_voices_list().unwrap();
    let mut names = Vec::new();
    for voice in &voices {
        names.push(voice.name.clone());
    }
    names
}
