// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    ssr: false,
    future: {
        compatibilityVersion: 4,
    },
    compatibilityDate: '2024-04-03',
    modules: [
        '@nuxtjs/tailwindcss',
        '@nuxt/eslint',
        'nuxt-time',
    ],
    runtimeConfig: {
        public: {
            // overridden with NUXT_PUBLIC_API_BASE
            apiBase: 'http://localhost:4000',
            // apiBase: 'https://my.jrnl.fm',
            appleClientId: 'fm.jrnl.jrnlapp',
            googleClientId: '909343102938-jr9m7fcgajq7r6op9phtq392juhikune.apps.googleusercontent.com'
        },
    },

    devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },
    vite: {
        clearScreen: false,
        envPrefix: ['VITE_', 'TAURI_'],
        server: {
            strictPort: true,
        },
        ssr: {
            noExternal: true
        }
    },
    experimental: {
        asyncContext: true
    }
});