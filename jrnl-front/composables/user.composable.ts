import type { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';

export const useUser = (userService: UserService) => {
    const localStorageUser = useLocalStorage<Partial<User>>('cached-user', {} as Partial<User>);

    const user = useState('user', () => localStorageUser.value);

    const refresh = async (ignoreError?: boolean) => {
        user.value = await userService.getMe(ignoreError);
        localStorageUser.value = user.value;
    };

    watch(user, () => {
        localStorageUser.value = user.value;
    }, { deep: true });

    return { refresh, user };
};