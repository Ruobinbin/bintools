<template>
  <el-button :loading="IsGptSovitsApiLoading" @click="startGptSovitsApi">
    {{ IsGptSovitsApiLoading ? "GPT-Sovits API 正在运行" : "启动 GPT-Sovits API" }}
  </el-button>

  <div>
    <el-button @click="fetchAudio">获取音频</el-button>
    <audio v-if="audioSrc" :src="audioSrc" controls></audio>
  </div>

  <div>
    <el-form :model="gptSovitsAudioRequestParams" label-width="150px" style="margin: 20px;">
      <el-form-item label="文本">
        <el-input v-model="gptSovitsAudioRequestParams.text"></el-input>
      </el-form-item>

      <el-form-item label="文本语言">
        <el-select v-model="gptSovitsAudioRequestParams.text_lang">
          <el-option label="全部按中文识别" value="all_zh"></el-option>
          <el-option label="全部按英文识别" value="en"></el-option>
          <el-option label="全部按日文识别" value="all_ja"></el-option>
          <el-option label="全部按粤语识别" value="all_yue"></el-option>
          <el-option label="全部按韩文识别" value="all_ko"></el-option>
          <el-option label="按中英混合识别" value="zh"></el-option>
          <el-option label="按日英混合识别" value="ja"></el-option>
          <el-option label="按粤英混合识别" value="yue"></el-option>
          <el-option label="按韩英混合识别" value="ko"></el-option>
          <el-option label="多语种启动切分识别语种" value="auto"></el-option>
          <el-option label="多语种启动切分识别语种（粤语）" value="auto_yue"></el-option>
        </el-select>
      </el-form-item>

      <el-form-item label="参考音频路径">
        <el-input v-model="gptSovitsAudioRequestParams.ref_audio_path"></el-input>
      </el-form-item>

      <el-form-item label="辅助参考音频路径">
        <el-select v-model="selectedAuxRefAudioPaths" multiple filterable clearable placeholder="选择辅助参考音频路径">
          <el-option v-for="path in allRefAudioPaths" :key="path" :label="path" :value="path" />
        </el-select>
      </el-form-item>

      <el-form-item label="提示文本">
        <el-input v-model="gptSovitsAudioRequestParams.prompt_text"></el-input>
      </el-form-item>

      <el-form-item label="提示语言">
        <el-select v-model="gptSovitsAudioRequestParams.prompt_lang">
          <el-option label="中文" value="zh"></el-option>
          <el-option label="英文" value="en"></el-option>
          <!-- 其他语言选项 -->
        </el-select>
      </el-form-item>

      <el-form-item label="Top-K">
        <el-input type="number" v-model="gptSovitsAudioRequestParams.top_k" min="1"></el-input>
      </el-form-item>

      <el-form-item label="Top-P">
        <el-input type="number" step="0.01" v-model="gptSovitsAudioRequestParams.top_p" min="0" max="1"></el-input>
      </el-form-item>

      <el-form-item label="Temperature">
        <el-input type="number" step="0.1" v-model="gptSovitsAudioRequestParams.temperature" min="0"></el-input>
      </el-form-item>

      <el-form-item label="文本拆分方法">
        <el-select v-model="gptSovitsAudioRequestParams.text_split_method" placeholder="选择文本拆分方法">
          <el-option label="不切（cut0）" value="cut0"></el-option>
          <el-option label="四句一切（cut1）" value="cut1"></el-option>
          <el-option label="50字一切（cut2）" value="cut2"></el-option>
          <el-option label="中文句号切（cut3）" value="cut3"></el-option>
          <el-option label="英文句号切（cut4）" value="cut4"></el-option>
          <el-option label="标点符号切（cut5）" value="cut5"></el-option>
        </el-select>
      </el-form-item>

      <el-form-item label="批次大小">
        <el-input type="number" v-model="gptSovitsAudioRequestParams.batch_size" min="1"></el-input>
      </el-form-item>

      <el-form-item label="批次阈值">
        <el-input type="number" step="0.01" v-model="gptSovitsAudioRequestParams.batch_threshold" min="0"
          max="1"></el-input>
      </el-form-item>

      <el-form-item label="拆分桶">
        <el-switch v-model="gptSovitsAudioRequestParams.split_bucket"></el-switch>
      </el-form-item>

      <el-form-item label="音频速度">
        <el-input type="number" step="0.1" v-model="gptSovitsAudioRequestParams.speed_factor" min="0"></el-input>
      </el-form-item>

      <el-form-item label="流式模式">
        <el-switch v-model="gptSovitsAudioRequestParams.streaming_mode"></el-switch>
      </el-form-item>

      <el-form-item label="种子">
        <el-input type="number" v-model="gptSovitsAudioRequestParams.seed"></el-input>
      </el-form-item>

      <el-form-item label="并行推理">
        <el-switch v-model="gptSovitsAudioRequestParams.parallel_infer"></el-switch>
      </el-form-item>

      <el-form-item label="重复惩罚">
        <el-input type="number" step="0.1" v-model="gptSovitsAudioRequestParams.repetition_penalty" min="0"></el-input>
      </el-form-item>

    </el-form>
  </div>

  <div v-if="currentGptModel" style="margin: 20px; padding: 10px; border: 1px solid #ddd; border-radius: 5px;">
    <h4>当前 GPT 模型路径:</h4>
    <p>{{ currentGptModel }}</p>
  </div>

  <div v-if="currentSovitsModel" style="margin: 20px; padding: 10px; border: 1px solid #ddd; border-radius: 5px;">
    <h4>当前 Sovits 模型路径:</h4>
    <p>{{ currentSovitsModel }}</p>
  </div>

  <div v-if="currentReferAudio" style="margin: 20px; padding: 10px; border: 1px solid #ddd; border-radius: 5px;">
    <h4>当前参考音频路径:</h4>
    <p>{{ currentReferAudio.path }}</p>
    <audio :src="convertFileSrc(currentReferAudio.originalPath)" controls></audio>
  </div>

  <div>
    <div v-for="model in models" :key="model.model_name"
      style="display: flex; flex-direction: column; margin: 10px; border: 1px solid #ddd; padding: 10px; border-radius: 5px;">
      <h3 style="margin: 0;">{{ model.model_name }}</h3>

      <div style="margin: 5px 0;">
        <strong>GPT 模型路径:</strong>
        <el-select v-model="selectedGptPath[model.model_name]" placeholder="选择 GPT 模型路径"
          @change="updateCurrentGptModel(model.model_name, $event)">
          <el-option v-for="path in model.gpt_model_paths" :key="path" :label="path" :value="path">
          </el-option>
        </el-select>
      </div>

      <div style="margin: 5px 0;">
        <strong>Sovits 模型路径:</strong>
        <el-select v-model="selectedSovitsPath[model.model_name]" placeholder="选择 Sovits 模型路径"
          @change="updateCurrentSovitsModel(model.model_name, $event)">
          <el-option v-for="path in model.sovits_model_paths" :key="path" :label="path" :value="path">
          </el-option>
        </el-select>
      </div>

      <div style="margin: 5px 0;">
        <strong>参考音频路径:</strong>
        <el-select v-model="selectedReferPath[model.model_name]" placeholder="选择参考音频路径"
          @change="updateCurrentReferAudio(model.model_name, $event)">
          <el-option v-for="path in model.ref_audio_paths" :key="path" :label="path" :value="path">
          </el-option>
        </el-select>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getFileNameFromPathWithoutExtension } from '../utils/defaultUtils'
import { ref, onMounted, computed, watch } from 'vue';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { ElMessage } from 'element-plus';
import { listen } from '@tauri-apps/api/event';

interface GptSovitsModel {
  model_name: string;
  gpt_model_paths: string[];
  sovits_model_paths: string[];
  ref_audio_paths: string[];
}

const models = ref<GptSovitsModel[]>([]);
const allRefAudioPaths = computed(() => {
  return models.value.flatMap(model =>
    model.ref_audio_paths.map(path => localPathToContainerPath(path))
  );
});
const selectedAuxRefAudioPaths = ref<string[]>([]);
const selectedGptPath = ref<Record<string, string | null>>({});
const selectedSovitsPath = ref<Record<string, string | null>>({});
const selectedReferPath = ref<Record<string, string | null>>({});
const currentGptModel = ref<string | null>(null);
const currentSovitsModel = ref<string | null>(null);
const currentReferAudio = ref<{ path: string; originalPath: string } | null>(null);

const IsGptSovitsApiLoading = ref(false);
const isGptSovitsApiRunning = ref(false);

const audioSrc = ref('');

const gptSovitsAudioRequestParams = ref({
  "text": "",                   // str. (必填) 要合成的文本
  "text_lang": "zh",              // str. (必填) 要合成文本的语言
  "ref_audio_path": "",         // str. (必填) 参考音频路径
  "aux_ref_audio_paths": [],    // list. (可选) 辅助参考音频路径，用于多说话人音调融合
  "prompt_text": "",            // str. (可选) 参考音频的提示文本
  "prompt_lang": "zh",            // str. (必填) 参考音频提示文本的语言
  "top_k": 5,                   // int. top k 采样
  "top_p": 1,                   // float. top p 采样
  "temperature": 1,             // float. 采样的温度
  "text_split_method": "cut0",  // str. 文本拆分方法，参见 text_segmentation_method.py 了解详细信息
  "batch_size": 1,              // int. 推理时的批处理大小
  "batch_threshold": 0.75,      // float. 批处理拆分的阈值
  "split_bucket": true,         // bool. 是否将批处理拆分成多个桶
  "speed_factor": 1.0,           // float. 控制合成音频的速度
  "streaming_mode": false,      // bool. 是否返回流式响应
  "seed": -1,                   // int. 随机种子，用于复现
  "parallel_infer": true,       // bool. 是否使用并行推理
  "repetition_penalty": 1.35    // float. T2S 模型的重复惩罚
});

watch(currentReferAudio, (newVal) => {
  if (!newVal) {
    return;
  }
  gptSovitsAudioRequestParams.value.ref_audio_path = newVal.path;
  gptSovitsAudioRequestParams.value.prompt_text = getFileNameFromPathWithoutExtension(newVal.path);
});


onMounted(async () => {
  listen('gpt_sovits_api_running', (event) => {
    isGptSovitsApiRunning.value = event.payload as boolean;
    if (isGptSovitsApiRunning.value) {
      ElMessage.success('GPT-Sovits API 启动成功');
    } else {
      ElMessage.error('GPT-Sovits API 启动失败');
    }
    IsGptSovitsApiLoading.value = false;
  });

  models.value = await invoke<GptSovitsModel[]>('get_gpt_sovits_models');
});

const fetchAudio = async () => {
  let audioBlob = await fetchAudioBlob(gptSovitsAudioRequestParams.value.text);
  if (!audioBlob) return;
  audioSrc.value = URL.createObjectURL(audioBlob);
};

const fetchAudioBlob = (text: string): Promise<Blob | null> => {
  if (!isGptSovitsApiRunning.value) {
    ElMessage.error('GPT-Sovits API 未运行');
    return Promise.resolve(null);
  }

  if (gptSovitsAudioRequestParams.value.ref_audio_path === '') {
    ElMessage.error('请选择参考音频路径');
    return Promise.resolve(null);
  }

  gptSovitsAudioRequestParams.value.text = text;
  if (gptSovitsAudioRequestParams.value.text === '') {
    ElMessage.error('请输入文本');
    return Promise.resolve(null);
  }

  return fetch('http://127.0.0.1:9880/tts', {
    method: 'POST',
    body: JSON.stringify(gptSovitsAudioRequestParams.value),
    headers: {
      'Content-Type': 'application/json',
    },
  })
    .then(response => {
      if (!response.ok) {
        return response.json().then(responseJson => {
          throw new Error(responseJson.message);
        });
      }
      return response.blob();
    })
    .catch(error => {
      ElMessage.error(`获取音频失败: ${error.message}`);
      return null;
    });
};
defineExpose({ fetchAudioBlob });


const localPathToContainerPath = (localPath: string) => {
  let linuxPath = localPath.replace(/\\/g, '/');
  const marker = 'gpt_sovits_model';
  const markerIndex = linuxPath.indexOf(marker);
  return linuxPath.substring(markerIndex);
};

const startGptSovitsApi = async () => {
  IsGptSovitsApiLoading.value = true;
  try {
    await invoke("start_gpt_sovits_api");
  } catch (error) {
    ElMessage.error(`启动 GPT-Sovits API 失败: ${error as string}`);
  } finally {
    IsGptSovitsApiLoading.value = false;
  }
};

const updateCurrentGptModel = async (modelName: string, path: string) => {
  const formattedPath = localPathToContainerPath(path);
  const success = await setGptModel(formattedPath);
  if (success) {
    for (const key in selectedGptPath.value) {
      if (key !== modelName) {
        selectedGptPath.value[key] = null;
      }
    }
    currentGptModel.value = formattedPath;
  } else {
    selectedGptPath.value[modelName] = null;
  }
};

const updateCurrentSovitsModel = async (modelName: string, path: string) => {
  const formattedPath = localPathToContainerPath(path);
  const success = await setSovitsModel(formattedPath);
  if (success) {
    for (const key in selectedSovitsPath.value) {
      if (key !== modelName) {
        selectedSovitsPath.value[key] = null;
      }
    }
    currentSovitsModel.value = formattedPath;
  } else {
    selectedSovitsPath.value[modelName] = null;
  }
};

const updateCurrentReferAudio = (modelName: string, path: string) => {
  for (const key in selectedReferPath.value) {
    if (key !== modelName) {
      selectedReferPath.value[key] = null;
    }
  }
  currentReferAudio.value = {
    path: localPathToContainerPath(path),
    originalPath: path
  };
};

const setGptModel = async (weightsPath: string): Promise<boolean> => {
  if (!isGptSovitsApiRunning.value) {
    ElMessage.error('GPT-Sovits API 未运行，无法设置模型');
    return false;
  }
  try {
    const response = await fetch(`http://127.0.0.1:9880/set_gpt_weights?weights_path=${encodeURIComponent(weightsPath)}`, {
      method: 'GET',
    });
    if (response.status === 200) {
      ElMessage.success('GPT 模型切换成功');
      return true;
    } else {
      const errorData = await response.json();
      ElMessage.error(`GPT 模型切换失败: ${errorData.message}`);
    }
  } catch (error) {
    ElMessage.error('请求失败');
  }
  return false;
};

const setSovitsModel = async (weightsPath: string): Promise<boolean> => {
  if (!isGptSovitsApiRunning.value) {
    ElMessage.error('GPT-Sovits API 未运行，无法设置模型');
    return false;
  }
  try {
    const response = await fetch(`http://127.0.0.1:9880/set_sovits_weights?weights_path=${encodeURIComponent(weightsPath)}`, {
      method: 'GET',
    });
    if (response.status === 200) {
      ElMessage.success('Sovits 模型切换成功');
      return true;
    } else {
      const errorData = await response.json();
      ElMessage.error(`Sovits 模型切换失败: ${errorData.message}`);
    }
  } catch (error) {
    ElMessage.error('请求失败');
  }
  return false;
};
</script>

<style></style>
