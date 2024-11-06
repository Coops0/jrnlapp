// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2024-04-03',
    devtools: { enabled: true },
    modules: ['@nuxtjs/tailwindcss', '@vueuse/nuxt', '@nuxt/fonts', '@nuxtjs/color-mode'],
    runtimeConfig: {
        public: {
            base: process.env.BASE_URL ? process.env.BASE_URL : 'http://localhost:3000',
            apiBase: process.env.API_BASE_URL ? process.env.API_BASE_URL : 'http://localhost:4000',
        }
    },
    colorMode: {
        preference: 'purple',
        fallback: 'purple',
        hid: 'nuxt-color-mode-script',
        globalName: '__NUXT_COLOR_MODE__',
        componentName: 'ColorScheme',
        classPrefix: '',
        classSuffix: '',
        storage: 'cookie',
        storageKey: 'theme-cache'
    },
    experimental: {
        sharedPrerenderData: true,
        asyncContext: true,
        normalizeComponentNames: true
    }
    // vite: {
    //     esbuild: {
    //         drop:  ['console'],
    //     }
    // }
});