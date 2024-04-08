import { defineConfig } from 'unocss';
import presetIcons from 'unocss/preset-icons';
import presetUno from 'unocss/preset-uno';
import presetWebFonts from 'unocss/preset-web-fonts';

export default defineConfig({
  content: {
    pipeline: {
      include: [/\.svelte|js|ts|html|css/]
    }
  },
  presets: [
    presetUno(),
    presetIcons({
      prefix: 'i-',
      extraProperties: {
        display: 'inline-block',
        'vertical-align': 'middle'
      }
    }),
    presetWebFonts({
      provider: 'bunny',
      fonts: {
        roboto: 'Roboto'
      }
    })
  ]
});
