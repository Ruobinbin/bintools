<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { isRegistered, register } from '@tauri-apps/plugin-global-shortcut';

const currentWindow = Window.getCurrent();
let inputElement = ref<HTMLInputElement>();
const input = ref("");
const handleInputEnter = async () => {
  switch (input.value) {
    case '/xs':
      await openWindow('novel', 'src/views/novel/novel.html');
      break;
    case '/setting':
      await openWindow('setting', 'src/views/setting/setting.html');
      break;
    default:
      console.log(await invoke("input_enter", { value: input.value }))
  }
}

onMounted(async () => {
  // 当 input 元素失去焦点时，隐藏窗口
  inputElement.value?.addEventListener("blur", async () => {
    await currentWindow.hide();
  });
  //快捷键控制窗口
  const shortcut = 'CommandOrControl+Shift+A';
  const isShortcutRegistered = await isRegistered(shortcut);

  if (!isShortcutRegistered) {
    await register(shortcut, async (event) => {
      if (event.state === 'Pressed') {
        if (await currentWindow.isVisible()) {
          await currentWindow.hide();
        } else {
          await currentWindow.show();
          await currentWindow.setFocus();
          inputElement.value?.focus();
        }
      }
    });
  }
})

const openWindow = async (windowLabel: string, path: string) => {
  const window = await Window.getByLabel(windowLabel);
  if (!window) {
    new WebviewWindow(windowLabel, {
      url: path
    });
  } else {
    await window.unminimize();
    await window.setFocus();
  }
};
</script>

<template>
  <input :style="{ height: '40px', width: '80%', margin: '0 auto', display: 'block' }" @keyup.enter="handleInputEnter"
    v-model="input" ref="inputElement" />
</template>

<style scoped></style>
