import { createApp } from "vue";
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import 'virtual:uno.css'

import App from "./App.vue";
import Novel from "./views/novel/novel.vue";
import Setting from "./views/setting/setting.vue";

const app = createApp(App);
app.use(ElementPlus);
if (document.getElementById("app")) {
    app.mount("#app");
}

const novel = createApp(Novel);
novel.use(ElementPlus);
if (document.getElementById("novel")) {
    novel.mount("#novel");
}

const setting = createApp(Setting);
setting.use(ElementPlus);
if (document.getElementById("setting")) {
    setting.mount("#setting");
}