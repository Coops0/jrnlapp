import type { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';

export const useUser = (userService: UserService | null) => {
    const localUser = useCookie<User | null>('cached-user');

    const user = useState<User | null>('user', () => localUser.value);

    const refresh = async () => {
        if (!userService) {
            throw new Error('userService is required for useUser.refresh');
        }

        user.value = await userService.getMe();
    };

    watch(user, u => {
        localUser.value = u;
    }, { deep: true });

    return { refresh, user };
};