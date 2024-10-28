export default defineNuxtPlugin(nuxtApp => {
    const session = useSupabaseSession();
    const config = useRuntimeConfig();

    const localApi = $fetch.create({
        baseURL: config.public.apiBase,
        onRequest({ options }) {
            const t = session.value?.access_token;
            if (t) {
                options.headers.set('Authorization', `Bearer ${t}`);
            }
        },
        async onResponseError({ options, response }) {
            if ((options as any).ignoreError) {
                return;
            }

            if (response.status === 401) {
                await nuxtApp.runWithContext(() => navigateTo('/login'));
            }
        }
    });

    return { provide: { localApi } };
});