import { UserService } from '~/services/user.service';

// flow:
// is authorized route:
//   yes -> does user have jwt?
//           yes -> does user have user data?
//                   yes -> PASS
//                   no -> refresh user data and PASS
//           no -> FAIL
//   no -> does user have jwt?
//           yes -> redirect to journal PASS
//           no -> PASS
export default defineNuxtRouteMiddleware(async (to, _from) => {
    const { $localApi } = useNuxtApp();
    const userService = new UserService($localApi);

    const { jwt } = useAuth();
    const { user, refresh } = useUser(userService);

    // if unset, redirect unauthenticated
    // if false, then redirect authenticated away
    const requiresAuth = to.meta['redirectUnautheticated'] !== false;

    if (!requiresAuth) {
        if (jwt.value) {
            return navigateTo('/current');
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