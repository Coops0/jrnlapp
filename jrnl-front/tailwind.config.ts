import type { Config } from 'tailwindcss';
import typography from '@tailwindcss/typography';
import { createThemes } from 'tw-colors';
// noinspection ES6PreferShortImport
import { themes } from './app/assets/themes';


export default <Partial<Config>>{
    content: [],
    plugins: [
        createThemes(themes),
        typography
    ],
    theme: {
        extend: {
            typography: (theme: ((key: string) => string)) => ({
                DEFAULT: {
                    css: {
                        color: theme('colors.text.500')
                    }
                }
            })
        }
    }
};

