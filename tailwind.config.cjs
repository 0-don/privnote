/** @type {import('tailwindcss').Config}*/
const config = {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			colors: {
				body: {
					DEFAULT: '#242729'
				}
			}
		}
	},

	plugins: []
};

module.exports = config;
