// noinspection JSUnusedGlobalSymbols
export default defineNuxtRouteMiddleware((to, _from) => {
    const { jwt } = useAuth();

    const requiresAuth = to.meta['requireLoggedIn'] === true;

    if (requiresAuth) {
        if (!jwt.value) {
            return navigateTo('/');
        }
    }
});