import type { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';

export const useUser = (userService: UserService) => {
    const localUser = useCookie<User>('cached-user');

    const user = useState('user', () => localUser.value);

    const refresh = async (ignoreError?: boolean) => {
        user.value = await userService.getMe(ignoreError);
        localUser.value = user.value;
    };

    watchDeep(user, () => {
        localUser.value = user.value;
    });

    return { refresh, user };
};