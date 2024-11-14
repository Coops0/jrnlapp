import type { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';
import { load } from '@tauri-apps/plugin-store';

export const useUser = async (userService: UserService | null) => {
    const localUser = await load('cached-user.json');
    const initialCachedUser = await localUser.get<User>('user');

    const user = useState<User | null>('user', () => initialCachedUser ?? null);

    const hasRefreshedRemotely = ref(false);

    const refresh = async () => {
        if (userService) {
            user.value = await userService.getMe();
            hasRefreshedRemotely.value = true;
        }
    };

    watch(user, u => {
        localUser.set('user', u);
    }, { deep: true });

    return { refresh, user, hasRefreshedRemotely };
};