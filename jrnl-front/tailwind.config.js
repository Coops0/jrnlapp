import { themes } from '~/assets/themes.js';

const { createThemes } = require('tw-colors');

/** @type {import('tailwindcss').Config} */
export default {
	content: [],
	plugins: [
		require('@tailwindcss/typography'),
		createThemes(themes)
	],
	theme: {
		extend: {
			fontFamily: {
				code: ['Fira Code', 'monospace']
			},
			typography: (theme) => ({
				DEFAULT: {
					css: {
						color: theme('colors.text.500')
					}
				}
			})
		}
	}
};

