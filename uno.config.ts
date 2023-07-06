import { defineConfig } from 'unocss';
import presetIcons from 'unocss/preset-icons';

export default defineConfig({
	content: {
		pipeline: {
			exclude: ['node_modules', '.git', '.vscode', 'build']
		}
	},
	presets: [
		presetIcons({
			prefix: 'i-',
			extraProperties: {
				display: 'inline-block',
				'vertical-align': 'middle'
			}
		})
	]
});
