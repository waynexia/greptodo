import { defineConfig, presetAttributify, presetUno, presetIcons, presetWebFonts } from 'unocss';

export default defineConfig({
    presets: [
        presetAttributify({ /* preset options */ }),
        presetUno(),
        presetIcons(),
        presetWebFonts({ fonts: { sans: ['Noto Serif'] } }),
    ],
});
