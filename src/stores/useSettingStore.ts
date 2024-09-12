// useSettingStore.ts
import { defineStore } from 'pinia';
import { getSetting, setSetting } from '../utils/dbUtils';

export const useSettingStore = defineStore('setting', {
    state: () => ({
        chatApiName: '',
        chatApiUrl: '',
        chatApiKey: '',
        chatModel: ''
    }),
    actions: {
        setChatApiName(name: string) {
            this.chatApiName = name;
            setSetting('chatApiName', name);
        },
        setChatApiUrl(url: string) {
            this.chatApiUrl = url;
            setSetting('chatApiUrl', url);
        },
        setChatApiKey(key: string) {
            this.chatApiKey = key;
            setSetting('chatApiKey', key);
        },
        setChatModel(model: string) {
            this.chatModel = model;
            setSetting('chatModel', model);
        },
        async init() {
            this.chatApiName = await getSetting('chatApiName') ?? '';
            this.chatApiUrl = await getSetting('chatApiUrl') ?? '';
            this.chatApiKey = await getSetting('chatApiKey') ?? '';
            this.chatModel = await getSetting('chatModel') ?? '';
        }
    }
});