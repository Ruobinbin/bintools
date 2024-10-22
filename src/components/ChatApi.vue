<template>
    <div>
        <el-form label-width="120px">
            <el-form-item label="API Name">
                <el-select v-model="currentChatApi.name" placeholder="选择一个API Name" @change="changeApiName">
                    <el-option v-for="api in apis" :key="api.id" :label="api.name" :value="api.name" />
                </el-select>
            </el-form-item>
            <el-form-item label="API URL">
                <el-input v-model="currentChatApi.url" placeholder="API URL" />
            </el-form-item>
            <el-form-item label="API Key">
                <el-input v-model="currentChatApi.apiKey" placeholder="API Key" />
            </el-form-item>
            <el-form-item label="选择模型">
                <el-select v-model="currentChatApi.model" placeholder="选择模型" @change="changeModel">
                    <el-option v-for="model in models" :key="model" :label="model" :value="model" />
                </el-select>
            </el-form-item>
            <el-form-item>
                <el-button type="primary" @click="showAddApiDialog">添加API</el-button>
                <el-button type="warning" @click="showUpdateApiDialog">修改API</el-button>
                <el-button type="danger" @click="deleteApi">删除API</el-button>
            </el-form-item>
        </el-form>



        <el-dialog v-model="apiDialogVisible" :title="dialogTitle">
            <el-form :model="{ dialogApiName, dialogApiUrl, dialogApiKey }" label-width="120px">
                <el-form-item label="API Name">
                    <el-input v-model="dialogApiName" placeholder="API Name" />
                </el-form-item>
                <el-form-item label="API URL">
                    <el-input v-model="dialogApiUrl" placeholder="API URL" />
                </el-form-item>
                <el-form-item label="API Key">
                    <el-input v-model="dialogApiKey" placeholder="API Key" />
                </el-form-item>
            </el-form>
            <span slot="footer" class="dialog-footer">
                <el-button @click="apiDialogVisible = false">取消</el-button>
                <el-button type="primary" @click="handleApiAction">{{ dialogAction }}</el-button>
            </span>
        </el-dialog>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { fetch } from '@tauri-apps/plugin-http';
import { ElMessage } from 'element-plus';
import { getAllLLMApis, addLLMApi, updateLLMApi, deleteLLMApi } from '../utils/dbUtils';

interface Api {
    id: number | null;
    name: string;
    url: string;
    api_key: string;
}

const currentChatApi = ref({
    name: '',
    url: '',
    apiKey: '',
    model: ''
});

const apis = ref<Api[]>([]);
const models = ref<string[]>([]);

const apiDialogVisible = ref(false);
const dialogTitle = ref('');
const dialogAction = ref('');
const dialogApiName = ref('');
const dialogApiUrl = ref('');
const dialogApiKey = ref('');

const showAddApiDialog = () => {
    dialogTitle.value = '添加API';
    dialogAction.value = '确定';
    dialogApiName.value = '';
    dialogApiUrl.value = '';
    dialogApiKey.value = '';
    apiDialogVisible.value = true;
};

const showUpdateApiDialog = () => {
    dialogTitle.value = '修改API';
    dialogAction.value = '确定';
    dialogApiName.value = currentChatApi.value.name;
    dialogApiUrl.value = currentChatApi.value.url;
    dialogApiKey.value = currentChatApi.value.apiKey;
    apiDialogVisible.value = true;
};

const handleApiAction = async () => {
    if (dialogTitle.value === '添加API') {
        await addApi();
    } else if (dialogTitle.value === '修改API') {
        await updateApi();
    }
};

const changeApiName = async () => {
    const { name = '', url = '', api_key = '' } = apis.value.find(api => api.name === currentChatApi.value.name) || {};
    currentChatApi.value.url = url;
    currentChatApi.value.apiKey = api_key;
    currentChatApi.value.model = "";
    localStorage.setItem('chatApiName', name);
    localStorage.setItem('chatApiUrl', url);
    localStorage.setItem('chatApiKey', api_key);
    localStorage.setItem('chatModel', "");
    await fetchModels();
};

const addApi = async () => {
    if (dialogApiName.value && dialogApiUrl.value && dialogApiKey.value) {
        await addLLMApi(dialogApiName.value, dialogApiUrl.value, dialogApiKey.value);
        apis.value = await getAllLLMApis();
        ElMessage.success('API 添加成功');
        apiDialogVisible.value = false;
    } else {
        ElMessage.error('请填写所有字段');
    }
};

const updateApi = async () => {
    const selected = apis.value.find(api => api.name === currentChatApi.value.name);
    if (selected && dialogApiName.value && dialogApiUrl.value && dialogApiKey.value) {
        await updateLLMApi(selected.id!, dialogApiName.value, dialogApiUrl.value, dialogApiKey.value);
        apis.value = await getAllLLMApis();
        localStorage.setItem('chatApiName', dialogApiName.value);
        localStorage.setItem('chatApiUrl', dialogApiUrl.value);
        localStorage.setItem('chatApiKey', dialogApiKey.value);
        currentChatApi.value.name = dialogApiName.value;
        currentChatApi.value.url = dialogApiUrl.value;
        currentChatApi.value.apiKey = dialogApiKey.value;
        ElMessage.success('API 修改成功');
        apiDialogVisible.value = false;
    }
};

const deleteApi = async () => {
    const selected = apis.value.find(api => api.name === currentChatApi.value.name);
    if (selected) {
        await deleteLLMApi(selected.id!);
        apis.value = await getAllLLMApis();
        localStorage.setItem('chatApiName', '');
        localStorage.setItem('chatApiUrl', '');
        localStorage.setItem('chatApiKey', '');
        currentChatApi.value.name = '';
        currentChatApi.value.url = '';
        currentChatApi.value.apiKey = '';
        ElMessage.success('API 删除成功');
    }
};

const fetchModels = async () => {
    fetch(currentChatApi.value.url + '/v1/models', {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${currentChatApi.value.apiKey}`
        }
    })
        .then(response => response.json())
        .then(data => {
            models.value = data.data.map((model: { id: string }) => model.id);
            ElMessage.success('模型列表获取成功');
        })
        .catch(error => {
            ElMessage.error('获取模型列表失败:' + error);
        });
};

const changeModel = async () => {
    localStorage.setItem('chatModel', currentChatApi.value.model);
};

onMounted(async () => {
    apis.value = await getAllLLMApis();
    currentChatApi.value.name = localStorage.getItem('chatApiName') ?? '';
    currentChatApi.value.url = localStorage.getItem('chatApiUrl') ?? '';
    currentChatApi.value.apiKey = localStorage.getItem('chatApiKey') ?? '';
    currentChatApi.value.model = localStorage.getItem('chatModel') ?? '';
});
</script>
