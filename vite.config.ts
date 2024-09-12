import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // Vite 选项专为 Tauri 开发定制，仅在 `tauri dev` 或 `tauri build` 中应用
  //
  // 1. 防止 vite 掩盖 rust 错误
  clearScreen: false,
  // 2. tauri 需要一个固定端口，如果该端口不可用则失败
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. 告诉 vite 忽略监视 `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    assetsDir: 'assets', // 设置静态资源目录 这个设置了之后构建前的目录和构建后的目录就保持一致了
    rollupOptions: {
      input: {
        index: 'index.html',
        novel: 'src/views/novel/novel.html',
        setting: 'src/views/setting/setting.html'
      },
    },
  }
}));
