// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2024-04-03',
    devtools: { enabled: true },
    modules: ['@nuxtjs/tailwindcss', '@vueuse/nuxt', '@nuxt/fonts', '@nuxtjs/color-mode'],
    runtimeConfig: {
        public: {
            base: process.env.BASE_URL ?? 'http://localhost:3000',
            apiBase: process.env.API_BASE_URL ?? 'http://localhost:4000',
        }
    },
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
        disableTransition: false
    },
    experimental: {
        sharedPrerenderData: true,
        asyncContext: true,
        normalizeComponentNames: true
    },
    vite: {
        esbuild: {
            drop: ['debugger'],
            pure: ['console.log', 'console.error', 'console.warn', 'console.debug', 'console.trace'],
        },
    }
});