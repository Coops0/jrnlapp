import type { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';

export const useUser = (userService: UserService | null) => {
    const localUser = useCookie<User | null>('cached-user');

    const user = useState<User | null>('user', () => localUser.value);

    const refresh = async (ignoreResponseError?: boolean) => {
        if (!userService) {
            throw new Error('userService is required for useUser');
        }

        user.value = await userService.getMe(ignoreResponseError);
        localUser.value = user.value;
    };

    watchDeep(user, () => {
        localUser.value = user.value;
    });

    return { refresh, user };
};