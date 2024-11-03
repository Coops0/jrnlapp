import Aura from '@primevue/themes/aura';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2024-04-03',
    devtools: { enabled: true },
    modules: ['@primevue/nuxt-module', '@nuxtjs/tailwindcss', '@vueuse/nuxt', '@nuxt/fonts'],
    runtimeConfig: {
        public: {
            base: process.env.BASE_URL ? process.env.BASE_URL : 'http://localhost:3000',
            apiBase: process.env.API_BASE_URL ? process.env.API_BASE_URL : 'http://localhost:4000',
        }
    },
    primevue: {
        options: {
            theme: {
                preset: Aura,
                options: {
                    darkModeSelector: true
                }
            },
            ripple: true
        }
    }
});