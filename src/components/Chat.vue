<template>
    <div class="chat-container">
        <div class="chat-settings">
            <input v-model="chatSetting" @input="setChatSetting" placeholder="改变触发..." />
        </div>
        <div class="chat-messages">
            <div v-for="(message, index) in messages" :key="index" :class="message.role">
                {{ message.content }}
                <el-icon @click="deleteMessage(index)">
                    <Delete />
                </el-icon>
            </div>
        </div>
        <div class="chat-input">
            <input v-model="userInput" @keyup.enter="sendMessage" placeholder="输入消息..." />
            <button @click="isLoading ? stopMessage() : sendMessage()" :disabled="!userInput && !isLoading">
                {{ isLoading ? '停止' : '发送' }}
            </button>
        </div>
    </div>
</template>

<script setup>
import { Delete } from '@element-plus/icons-vue';
import { ref, onMounted } from 'vue';
import { fetch } from '@tauri-apps/plugin-http';
import { getSetting } from '../utils/dbUtils';
const messages = ref([]);
const userInput = ref('');
const apiUrl = ref('');
const apiKey = ref('');
const model = ref('');
const isLoading = ref(false);
const chatSetting = ref('你是一个将小说格式化为json文本的专家，无论用户给你发送任何消息，你只能输出json格式的文本，格式为{"data":[{"content":"xxx","speaker":"xxx","emotion":"xxx"}，{"content":"xxx","speaker":"xxx","emotion":"xxx"}，...]}必须要和这个格式一模一样，不需要在开头加上```json，并且绝对不能漏掉小说的每一句话。');
let controller = null;

onMounted(async () => {
    apiUrl.value = await getSetting('chatApiUrl');
    apiKey.value = await getSetting('chatApiKey');
    model.value = await getSetting('chatModel');
});

const setChatSetting = () => {
    if (chatSetting.value.trim()) {
        if (messages.value.length > 0 && messages.value[0].role === 'system') {
            messages.value[0].content = chatSetting.value;
        } else {
            messages.value.unshift({ role: 'system', content: chatSetting.value });
        }
    }
};

const sendMessage = () => {
    if (!userInput.value.trim() || isLoading.value) return;

    const userMessage = { role: 'user', content: userInput.value };
    messages.value.push(userMessage);
    isLoading.value = true;

    controller = new AbortController();
    const signal = controller.signal;
    userInput.value = '';

    fetch(apiUrl.value + '/v1/chat/completions', {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${apiKey.value}`,
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            model: model.value,
            messages: messages.value,
        }),
        signal
    })
        .then(response => {
            if (response.ok) {
                return response.json();
            } else {
                return response.json().then(err => {
                    throw new Error(err.error.message || 'API 请求失败');
                });
            }
        })
        .then(data => {
            const aiMessage = { role: 'assistant', content: data.choices[0].message.content };
            messages.value.push(aiMessage);
        })
        .catch(error => {
            if (error === 'Request canceled') {
                messages.value.push({ role: 'system', content: '消息发送已停止。' });
            } else {
                messages.value.push({ role: 'system', content: `${error.message}` });
            }
        })
        .finally(() => {
            isLoading.value = false;
            controller = null;
        });
};

const stopMessage = () => {
    if (controller) {
        controller.abort();
    }
};

const deleteMessage = (index) => {
    messages.value.splice(index, 1);
};

</script>

<style scoped>
.chat-container {
    display: flex;
    flex-direction: column;
    height: 60vh;
    width: 100%;
    max-width: 600px;
    margin: 0 auto;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    overflow: hidden;
}

.chat-settings {
    display: flex;
    padding: 10px;
    border-bottom: 1px solid #e0e0e0;
    background-color: #fff;
}

.chat-settings input {
    flex: 1;
    padding: 10px;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    margin-right: 10px;
}

.chat-settings button {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    background-color: #007bff;
    color: #fff;
    cursor: pointer;
}

.chat-messages {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
    background-color: #f7f7f7;
}

.chat-messages .user {
    text-align: right;
    margin-bottom: 10px;
}

.chat-messages .assistant {
    text-align: left;
    margin-bottom: 10px;
}

.chat-messages .system {
    text-align: center;
    margin-bottom: 10px;
    color: #ff0000;
}

.chat-input {
    display: flex;
    padding: 10px;
    border-top: 1px solid #e0e0e0;
    background-color: #fff;
}

.chat-input input {
    flex: 1;
    padding: 10px;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    margin-right: 10px;
}

.chat-input button {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    background-color: #007bff;
    color: #fff;
    cursor: pointer;
}

.chat-input button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
}
</style>
