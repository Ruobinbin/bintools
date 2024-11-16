import { defineConfig, presetUno, presetAttributify } from 'unocss'

export default defineConfig({
    presets: [
        presetUno(),
        presetAttributify()// 属性化模式
    ]
})