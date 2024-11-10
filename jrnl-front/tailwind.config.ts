import typography from '@tailwindcss/typography';
import { createThemes } from 'tw-colors';
// noinspection ES6PreferShortImport
import { themes } from './app/assets/themes';

export default {
    content: [],
    plugins: [
        // @ts-expect-error - type definitions aren't valid between these 2
        createThemes(themes),
        typography
    ]
};

