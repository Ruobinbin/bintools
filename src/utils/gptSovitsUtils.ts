import { invoke } from "@tauri-apps/api/core";
import { fetch } from "@tauri-apps/plugin-http";

export let isGptSovitsStart = false;

export interface IRefAudio {
    path: string;
    content: string;
    emotion: string;
    language: string;
}

export interface IGptSovitsModel {
    model_name: string;
    gpt_model_paths: string[];
    sovits_model_paths: string[];
    ref_audios: IRefAudio[];
}

export interface ITTSRequestParams {
    text: string;                   // 要合成的文本
    text_lang: string;              // 要合成文本的语言
    ref_audio_path: string;         // 参考音频路径
    aux_ref_audio_paths: string[];  // 辅助参考音频路径，用于多说话人音调融合
    prompt_text: string;            // 参考音频的提示文本
    prompt_lang: string;            // 参考音频提示文本的语言
    top_k: number;                  // top k 采样
    top_p: number;                  // top p 采样
    temperature: number;            // 采样的温度
    text_split_method: string;      // 文本拆分方法
    batch_size: number;             // 推理时的批处理大小
    batch_threshold: number;        // 批处理拆分的阈值
    split_bucket: boolean;          // 是否将批处理拆分成多个桶
    speed_factor: number;           // 控制合成音频的速度
    streaming_mode: boolean;        // 是否返回流式响应
    seed: number;                   // 随机种子，用于复现
    parallel_infer: boolean;        // 是否使用并行推理
    repetition_penalty: number;     // T2S 模型的重复惩罚
}

export const localPathToContainerPath = (localPath: string) => {
    if (!localPath) {
        throw new Error('localPath 不能为空');
    }
    let linuxPath = localPath.replace(/\\/g, '/');
    const marker = 'gpt_sovits_model';
    const markerIndex = linuxPath.indexOf(marker);
    if (markerIndex === -1) {
        throw new Error(`路径中未找到标记: ${marker}`);
    }
    return linuxPath.substring(markerIndex);
};

export const fetchAudioBlob = async (params: ITTSRequestParams): Promise<Blob> => {
    if (!isGptSovitsStart) {
        throw new Error('GPT-Sovits API 未运行');
    }

    const response = await fetch('http://127.0.0.1:9880/tts', {
        method: 'POST',
        body: JSON.stringify(params),
        headers: {
            'Content-Type': 'application/json',
        },
    });

    if (!response.ok) {
        const errorData = await response.json();
        throw new Error(`获取音频失败: ${errorData.message}`);
    }

    return response.blob();
};


export async function getGptSovitsModels(): Promise<IGptSovitsModel[]> {
    const models = await invoke<IGptSovitsModel[]>('get_gpt_sovits_models');
    return models;
}

export const startGptSovitsApi = async () => {
    await invoke('start_gpt_sovits_api').then(() => {
        isGptSovitsStart = true;
    }).catch(error => {
        throw new Error(`启动 GPT-Sovits API 失败: ${error as string}`);
    });
};

export const setGptModel = async (weightsPath: string) => {
    return fetch(`http://127.0.0.1:9880/set_gpt_weights?weights_path=${encodeURIComponent(weightsPath)}`, {
        method: 'GET',
    })
        .then(response => {
            if (response.status !== 200) {
                return response.json().then(errorData => {
                    throw new Error(`GPT 模型切换失败: ${errorData.message}`);
                });
            }
        })
        .catch(error => {
            throw new Error(`请求失败: ${error.message}`);
        });
};

export const setSovitsModel = async (weightsPath: string) => {
    return fetch(`http://127.0.0.1:9880/set_sovits_weights?weights_path=${encodeURIComponent(weightsPath)}`, {
        method: 'GET',
    })
        .then(response => {
            console.log(response.status)
            if (response.status !== 200) {
                return response.json().then(errorData => {
                    throw new Error(`Sovits 模型切换失败: ${errorData.message}`);
                });
            }
        })
        .catch(error => {
            throw new Error(`请求失败: ${error.message}`);
        });
};


// class GptSovitsModel implements IGptSovitsModel {
//     constructor(
//         public modelName: string,
//         public gptModelPaths: string[],
//         public sovitsModelPaths: string[],
//         public refAudios: IRefAudio[]
//     ) { }
// }

// class RefAudio implements IRefAudio {
//     constructor(
//         public path: string,
//         public content: string,
//         public emotion: string,
//         public language: string
//     ) { }
// }