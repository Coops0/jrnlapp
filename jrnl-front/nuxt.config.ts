// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    future: {
        compatibilityVersion: 4,
    },
    compatibilityDate: '2024-04-03',
    modules: [
        '@nuxtjs/tailwindcss',
        '@nuxtjs/color-mode',
        '@nuxt/eslint',
        'nuxt-time',
    ],
    colorMode: {
        preference: 'lunar_placeholder',
        fallback: 'lunar_placeholder',
        hid: 'nuxt-color-mode-script',
        globalName: '__NUXT_COLOR_MODE__',
        componentName: 'ColorScheme',
        classPrefix: '',
        classSuffix: '',
        storage: 'cookie',
        storageKey: 'theme-cache',
        disableTransition: false,
    },
    runtimeConfig: {
        public: {
            // overridden with NUXT_PUBLIC_API_BASE
            apiBase: 'http://localhost:4000',
            // apiBase: 'https://my.jrnl.fm',
            appleClientId: 'fm.jrnl.oauth-signin',
            googleClientId: '909343102938-jr9m7fcgajq7r6op9phtq392juhikune.apps.googleusercontent.com'
        },
    },
    vite: process.env.NODE_ENV === 'production' ? {
        esbuild: {
            drop: ['debugger', 'console'],
            // pure: ['console.log', 'console.error', 'console.warn', 'console.debug', 'console.trace'],
        },
    } : {}
});