// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2024-04-03',
    devtools: { enabled: true },
    modules: ['@nuxtjs/tailwindcss', '@vueuse/nuxt', '@nuxtjs/color-mode', '@nuxt/eslint'],
    colorMode: {
        preference: 'lunar',
        fallback: 'lunar',
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
            apiBase: process.env.API_BASE_URL ?? 'http://localhost:4000',
        },
    },
    experimental: {
        sharedPrerenderData: true,
        asyncContext: true,
        normalizeComponentNames: true,
    },
    vite: process.env.NODE_ENV === 'production' ? {
        esbuild: {
            drop: ['debugger'],
            pure: ['console.log', 'console.error', 'console.warn', 'console.debug', 'console.trace'],
        },
    } : {},
});
