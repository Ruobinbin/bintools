use super::default_utils;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct GptSovitsModel {
    model_name: String,
    gpt_model_paths: Vec<String>,
    sovits_model_paths: Vec<String>,
    ref_audio_paths: Vec<String>,
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

        let wav_files: Vec<_> = default_utils::get_files_with_extension(&dir_path, "wav");
        let ref_audio_paths: Vec<_> = wav_files
            .into_iter()
            .map(|wav_path| wav_path.display().to_string())
            .collect();

        let model = GptSovitsModel {
            model_name,
            gpt_model_paths,
            sovits_model_paths,
            ref_audio_paths,
        };

        models.push(model);
    }
    models
}
