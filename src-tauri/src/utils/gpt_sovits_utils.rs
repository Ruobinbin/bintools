use super::default_utils;
use serde::{Deserialize, Serialize};
use std::path::Path;
#[derive(Serialize, Deserialize, Debug)]
pub struct RefAudio {
    path: String,
    content: String,
    emotion: String,
    language: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GptSovitsModel {
    model_name: String,
    gpt_model_paths: Vec<String>,
    sovits_model_paths: Vec<String>,
    ref_audios: Vec<RefAudio>,
}

pub fn get_gpt_sovits_models(base_dir: &Path) -> Vec<GptSovitsModel> {
    let mut models = Vec::new();

    let dir_path_list = default_utils::get_dir_in_dir(base_dir);

    for dir_path in dir_path_list {
        let model_name = default_utils::get_dir_or_file_name(&dir_path);

        let gpt_model_paths: Vec<_> = default_utils::get_files_with_extension(&dir_path, "ckpt")
            .into_iter()
            .map(|path| path.display().to_string())
            .collect();

        let sovits_model_paths: Vec<_> = default_utils::get_files_with_extension(&dir_path, "pth")
            .into_iter()
            .map(|path| path.display().to_string())
            .collect();

        let ref_audios: Vec<_> = default_utils::get_files_with_extension(&dir_path, "wav")
            .into_iter()
            .map(|wav_path| {
                let file_name = default_utils::get_file_name_without_extension(&wav_path);
                let parts: Vec<&str> = file_name.split('_').collect();
                let emotion = parts.get(0).unwrap_or(&"").to_string();
                let language = parts.get(1).unwrap_or(&"").to_string();
                let content = parts.get(2).unwrap_or(&"").to_string();
                RefAudio {
                    path: wav_path.display().to_string(),
                    content,
                    emotion,
                    language,
                }
            })
            .collect();

        let model = GptSovitsModel {
            model_name,
            gpt_model_paths,
            sovits_model_paths,
            ref_audios: ref_audios,
        };

        models.push(model);
    }
    models
}
