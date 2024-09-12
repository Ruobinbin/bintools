<template>
    <div>
        <el-form :model="settingStore" label-width="120px">
            <el-form-item label="API Name">
                <el-select v-model="settingStore.chatApiName" placeholder="选择一个API Name" @change="changeApiName">
                    <el-option v-for="api in apis" :key="api.id" :label="api.name" :value="api.name" />
                </el-select>
            </el-form-item>
            <el-form-item label="API URL">
                <el-input v-model="settingStore.chatApiUrl" placeholder="API URL" />
            </el-form-item>
            <el-form-item label="API Key">
                <el-input v-model="settingStore.chatApiKey" placeholder="API Key" />
            </el-form-item>
            <el-form-item label="选择模型">
                <el-select v-model="settingStore.chatModel" placeholder="选择模型" @change="changeModel">
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
import { useSettingStore } from '../stores/useSettingStore';

interface Api {
    id: number | null;
    name: string;
    url: string;
    api_key: string;
}

const apis = ref<Api[]>([]);
const models = ref<string[]>([]);

const apiDialogVisible = ref(false);
const dialogTitle = ref('');
const dialogAction = ref('');
const dialogApiName = ref('');
const dialogApiUrl = ref('');
const dialogApiKey = ref('');

const settingStore = useSettingStore();

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
    dialogApiName.value = settingStore.chatApiName;
    dialogApiUrl.value = settingStore.chatApiUrl;
    dialogApiKey.value = settingStore.chatApiKey;
    apiDialogVisible.value = true;
};

const handleApiAction = async () => {
    if (dialogTitle.value === '添加API') {
        await addApi();
    } else if (dialogTitle.value === '修改API') {
        await updateApi();
    }
};

const changeApiName = () => {
    const { name = '', url = '', api_key = '' } = apis.value.find(api => api.name === settingStore.chatApiName) || {};
    settingStore.setChatApiName(name);
    settingStore.setChatApiUrl(url);
    settingStore.setChatApiKey(api_key);
    settingStore.setChatModel("");
    fetchModels();
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
    const selected = apis.value.find(api => api.name === settingStore.chatApiName);
    if (selected && dialogApiName.value && dialogApiUrl.value && dialogApiKey.value) {
        await updateLLMApi(selected.id!, dialogApiName.value, dialogApiUrl.value, dialogApiKey.value);
        apis.value = await getAllLLMApis();
        settingStore.setChatApiName(dialogApiName.value);
        settingStore.setChatApiUrl(dialogApiUrl.value);
        settingStore.setChatApiKey(dialogApiKey.value);
        ElMessage.success('API 修改成功');
        apiDialogVisible.value = false;
    }
};

const deleteApi = async () => {
    const selected = apis.value.find(api => api.name === settingStore.chatApiName);
    if (selected) {
        await deleteLLMApi(selected.id!);
        apis.value = await getAllLLMApis();
        settingStore.setChatApiName('');
        settingStore.setChatApiUrl('');
        settingStore.setChatApiKey('');
        ElMessage.success('API 删除成功');
    }
};

const fetchModels = () => {
    fetch(settingStore.chatApiUrl + '/v1/models', {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${settingStore.chatApiKey}`
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

const changeModel = () => {
    settingStore.setChatModel(settingStore.chatModel);
};

onMounted(async () => {
    apis.value = await getAllLLMApis();
    await settingStore.init();
    fetchModels();
});
</script>
