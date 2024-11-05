import { UserService } from '~/services/user.service';

export default defineNuxtRouteMiddleware(async (to, from) => {
    const { $localApi } = useNuxtApp();
    const userService = new UserService($localApi);

    const { jwt } = useAuth();
    const { user, refresh } = useUser(userService);

    // if unset, redirect unauthenticated
    // if false, then redirect authenticated away
    const requiresAuth = to.meta['redirectUnautheticated'] !== false;

    console.debug(`middleware: jwt ${!!jwt.value}, user id ${user.value?.id}, requiresAuth ${requiresAuth}`);
    if (!requiresAuth) {
        if (jwt.value) {
            return navigateTo('/page');
        } else {
            return;
        }
    }

    if (!jwt.value) {
        return navigateTo('/login');
    }

    if (!user.value?.id) {
        await refresh();
    }

    if (!user.value?.id) {
        return navigateTo('/login');
    }
});