import Aura from '@primevue/themes/aura';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2024-04-03',
    devtools: { enabled: true },
    modules: ['@primevue/nuxt-module', '@nuxtjs/tailwindcss', '@nuxtjs/supabase', '@vueuse/nuxt', '@nuxt/fonts'],
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
    },
    supabase: {
        url: process.env.NUXT_PUBLIC_SUPABASE_URL,
        key: process.env.NUXT_PUBLIC_SUPABASE_KEY,
        // serviceKey: process.env.SUPABASE_SERVICE_KEY,
        redirect: true,
        redirectOptions: {
            login: '/login',
            callback: '/confirm',
            // include: ['*'],
            exclude: ['/login', '/confirm'],
            cookieRedirect: true,
        }
    }
});