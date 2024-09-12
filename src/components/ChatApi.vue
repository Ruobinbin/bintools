<template>
    <div>
        <el-select v-model="selectedApiUrl" placeholder="选择一个API URL" @change="changeApiUrl">
            <el-option v-for="api in apis" :key="api.url" :label="api.url" :value="api.url" />
        </el-select>
        <el-input v-model="selectedApiKey" placeholder="API Key" />
        <el-select v-model="selectedModel" placeholder="选择模型" @change="changeModel">
            <el-option v-for="model in models" :key="model.id" :label="model.id" :value="model.id" />
        </el-select>
        <el-button type="primary" @click="addApi">添加API</el-button>
        <el-button type="warning" @click="updateApi">修改API</el-button>
        <el-button type="danger" @click="deleteApi">删除API</el-button>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { fetch } from '@tauri-apps/plugin-http';
import { ElMessageBox, ElMessage } from 'element-plus';
import { getAllLLMApis, getSetting, setSetting, updateLLMApi, deleteLLMApi, addLLMApi } from '../utils/dbUtils';

const apis = ref<Array<{ url: string, api_key: string }>>([]);
const selectedApiUrl = ref<string>('');
const selectedApiKey = ref<string>('');
const models = ref<Array<{ id: string, object: string, owned_by: string }>>([]);
const selectedModel = ref<string>('');

const changeApiUrl = () => {
    const selectedApi = apis.value.find(api => api.url === selectedApiUrl.value);
    selectedApiKey.value = selectedApi ? selectedApi.api_key : '';
    setSetting('chatApiUrl', selectedApiUrl.value);
    setSetting('chatApiKey', selectedApiKey.value);
    fetchModels();
};

const addApi = () => {
    ElMessageBox.prompt('请输入新的API URL', '添加API URL', {
        confirmButtonText: '下一步',
        cancelButtonText: '取消',
    }).then(({ value: url }) => {
        if (url) {
            ElMessageBox.prompt('请输入API Key', '添加API Key', {
                confirmButtonText: '确定',
                cancelButtonText: '取消',
            }).then(async ({ value: apiKey }) => {
                if (apiKey) {
                    await addLLMApi(url, apiKey);
                    apis.value = await getAllLLMApis();
                    selectedApiUrl.value = url;
                    selectedApiKey.value = apiKey;
                    setSetting('chatApiUrl', selectedApiUrl.value);
                    setSetting('chatApiKey', selectedApiKey.value);
                    ElMessage.success('API 添加成功');
                }
            });
        }
    });
};

const updateApi = async () => {
    if (selectedApiUrl.value && selectedApiKey.value) {
        await updateLLMApi(selectedApiUrl.value, selectedApiKey.value);
        apis.value = await getAllLLMApis();
        ElMessage.success('API 修改成功');
    }
};

const deleteApi = async () => {
    if (selectedApiUrl.value) {
        await deleteLLMApi(selectedApiUrl.value);
        apis.value = await getAllLLMApis();
        selectedApiUrl.value = '';
        selectedApiKey.value = '';
        setSetting('chatApiUrl', '');
        setSetting('chatApiKey', '');
        ElMessage.success('API 删除成功');
    }
};

const fetchModels = async () => {
    try {
        const response = await fetch(selectedApiUrl.value + '/v1/models', {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${selectedApiKey.value}`
            }
        });
        const data = await response.json();
        models.value = data.data;
    } catch (error) {
        ElMessage.error('获取模型列表失败');
    }
};

const changeModel = () => {
    setSetting('chatModel', selectedModel.value);
};


onMounted(async () => {
    apis.value = await getAllLLMApis();
    selectedApiUrl.value = (await getSetting('chatApiUrl')) || '';
    selectedApiKey.value = (await getSetting('chatApiKey')) || '';
    selectedModel.value = (await getSetting('chatModel')) || '';
    await fetchModels();
});
</script>
