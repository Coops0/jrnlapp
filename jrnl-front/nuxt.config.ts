// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    future: {
        compatibilityVersion: 4,
    },
    compatibilityDate: '2024-04-03',
    devtools: { enabled: true },
    modules: ['@nuxtjs/tailwindcss', '@nuxtjs/color-mode', '@nuxt/eslint', 'nuxt-time', '@nuxt/scripts'],
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
            // apiBase: 'http://localhost:4000',
            apiBase: 'https://my.jrnl.fm',
            appleClientId: 'fm.jrnl.jrnlapp',
        },
    },
    vite: process.env.NODE_ENV === 'production' ? {
        esbuild: {
            drop: ['debugger'],
            pure: ['console.log', 'console.error', 'console.warn', 'console.debug', 'console.trace'],
        },
    } : {},
});