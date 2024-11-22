// https://nuxt.com/docs/api/configuration/nuxt-config
import { fileURLToPath } from 'url';

export default defineNuxtConfig({
    ssr: false,
    future: {
        compatibilityVersion: 4,
    },
    compatibilityDate: '2024-04-03',
    alias: {
      '#shared': fileURLToPath(new URL('../jrnl-front-shared', import.meta.url))
    },
    modules: [
        '@nuxtjs/tailwindcss',
        '@nuxt/eslint',
        'nuxt-time',
    ],
    runtimeConfig: {
        public: {
            // apiBase: 'https://my.jrnl.fm',
            apiBase: process.env.TAURI_DEV_HOST ? 'http://' + process.env.TAURI_DEV_HOST + ':4000' : 'http://localhost:4000',
            appleClientId: 'fm.jrnl.oauth-signin',
            googleClientId: '909343102938-jr9m7fcgajq7r6op9phtq392juhikune.apps.googleusercontent.com'
        },
    },

    devServer: {
        host: process.env.TAURI_DEV_HOST || 'localhost'
    },
    vite: {
        clearScreen: false,
        envPrefix: ['VITE_', 'TAURI_'],
        server: {
            strictPort: true,
            watch: {
                ignored: ['./src-tauri/**'],
            }
        },
        ssr: {
            noExternal: true
        }
    },
    hooks: {
        'vite:extend': function ({ config }) {
            if (config.server && config.server.hmr && config.server.hmr !== true) {
                config.server.hmr.protocol = 'ws';
                config.server.hmr.host = process.env.TAURI_DEV_HOST || 'localhost';
                config.server.hmr.port = 3000;
            }
        },
    },
});