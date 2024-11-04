import { useAuth } from '~/composables/auth.composable';

export default defineNuxtPlugin(nuxtApp => {
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
            if ((options as any).ignoreError) {
                return;
            }

            if (response.status === 401) {
                console.debug('got unauthorized response in $api plugin, redirecting to login');
                await nuxtApp.runWithContext(() => navigateTo('/login'));
            }
        }
    });

    return { provide: { localApi } };
});