import { useUser } from '~/composables/user.composable';
import { UserService } from '~/services/user.service';

export default defineNuxtRouteMiddleware(async (to, from) => {
    const { $localApi } = useNuxtApp();
    const userService = new UserService($localApi);

    const { user, refresh } = useUser(userService);

    // if unset, redirect unauthenticated
    // if false, then redirect authenticated away
    const requiresAuth = to.meta['requiresAuth'] !== false;

    if (requiresAuth) {
        if (!user.value?.id) {
            await refresh();
        }

        if (!user.value?.id) {
            return navigateTo('/login');
        }
    } else {
        if (user.value?.id) {
            return navigateTo('/');
        }
    }
});