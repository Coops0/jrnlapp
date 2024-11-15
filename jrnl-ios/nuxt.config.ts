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
            // apiBase: 'http://localhost:4000',
            // apiBase: 'https://my.jrnl.fm',
            apiBase: process.env.TAURI_DEV_HOST + ':4000' || 'https://my.jrnl.fm',
            appleClientId: 'fm.jrnl.oauth-signin',
            googleClientId: '909343102938-jr9m7fcgajq7r6op9phtq392juhikune.apps.googleusercontent.com'
        },
    },

    devServer: {
        host: process.env.TAURI_DEV_HOST || 'localhost'
        // host: process.env.TAURI_DEV_HOST || '0.0.0.0'
    },
    vite: {
        clearScreen: false,
        envPrefix: ['VITE_', 'TAURI_'],
        server: {
            strictPort: true,
            watch: {
                ignored: ['./src-tauri/**'],
            },
        },
        ssr: {
            noExternal: true
        }
    },
    experimental: {
        asyncContext: true
    },
    hooks: {
        'vite:extend': function ({config}) {
            if (config.server && config.server.hmr && config.server.hmr !== true) {
                config.server.hmr.protocol = 'ws';
                config.server.hmr.host = process.env.TAURI_DEV_HOST || 'localhost';
                config.server.hmr.port = 3000;
            }
        },
    },
});