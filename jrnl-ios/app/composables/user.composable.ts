import type { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';
import { useLocalStorage } from '~/composables/util/local-storage.util.composable';

export const useUser = (userService: UserService | null) => {
    const user = useLocalStorage<User | null>('user', () => null);

    const hasRefreshedRemotely = ref(false);

    const refresh = async () => {
        if (userService) {
            user.value = await userService.getMe();
            hasRefreshedRemotely.value = true;
        }
    };

    return { refresh, user, hasRefreshedRemotely };
};