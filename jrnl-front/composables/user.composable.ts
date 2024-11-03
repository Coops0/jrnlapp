import type { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';

export const useUser = (userService: UserService) => {
    const localStorageUser = useLocalStorage<Partial<User>>('cached-user', {} as Partial<User>);

    const user = useState('user', () => localStorageUser.value);

    const refresh = async () => {
        user.value = await userService.getMe();
        localStorageUser.value = user.value;
    };

    const updateStorage = () => {
        localStorageUser.value = user.value;
    }

    return { refresh, user, updateStorage };
};