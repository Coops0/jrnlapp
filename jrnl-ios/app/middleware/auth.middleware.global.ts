// noinspection JSUnusedGlobalSymbols
export default defineNuxtRouteMiddleware(async (to, _from) => {
    const { jwt } = await useAuth();

    const requiresAuth = to.meta['requireLoggedIn'] === true;

    if (requiresAuth) {
        if (!jwt.value) {
            return navigateTo('/');
        }
    }
});