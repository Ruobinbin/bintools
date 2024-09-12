import { createApp } from "vue";
import App from "./setting.vue";

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { createPinia } from 'pinia';

const app = createApp(App);
app.use(ElementPlus);
app.use(createPinia());
app.mount("#app");
