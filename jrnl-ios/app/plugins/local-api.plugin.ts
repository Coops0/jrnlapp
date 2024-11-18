import { useAuth } from '~/composables/auth.composable';

export default defineNuxtPlugin(_nuxtApp => {
    const config = useRuntimeConfig();
    const { jwt } = useAuth();

    const localApi = $fetch.create({
        credentials: 'include',
        baseURL: config.public.apiBase,
        onRequest({ options }) {
            const j = jwt.value;
            if (j?.length) {
                options.headers.set('Authorization', `Bearer ${j}`);
            }
        },
        async onResponseError({ options, response }) {
            if (!options.ignoreResponseError && response.status === 401) {
                jwt.value = null;
            }
        }
    });

    return { provide: { localApi } };
});