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
            await refresh(true);
        }

        if (!user.value?.id) {
            return navigateTo('/login');
        }

        // we're good
        return;
    }

    // if we came from a page that required auth, then let's try
    if (from.name && from.meta['requiresAuth'] !== false) {
        await refresh(true);
    }

    if (user.value?.id) {
        return navigateTo('/page');
    }

    // we're good, stay on your unauthenticated page
});