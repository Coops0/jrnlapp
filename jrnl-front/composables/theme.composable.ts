import { UserService } from '~/services/user.service';
import { useUser } from '~/composables/user.composable';

export const useTheme = (userService: UserService | null) => {
    const { user } = useUser(userService);

    const cachedTheme = useCookie<string>('theme');
    const theme = useState('theme', () => user.value?.theme || cachedTheme.value || 'purple');

    watchImmediate(theme, t => {
        cachedTheme.value = t;
    });

    useColorMode({
        storageKey: null,
        storageRef: theme,
        initialValue: user.value?.theme ?? 'purple',
        attribute: 'data-theme',
        modes: {
            purple: 'purple',
            plant: 'plant'
        }
    });

    watchImmediate(user, p => {
        if (p?.theme && p.theme !== theme.value) {
            console.debug('useTheme: user theme changed, setting theme to', p.theme);
            theme.value = p.theme;
        }
    }, { deep: true });

    async function setTheme(name: string) {
        theme.value = name;
        if (!userService) {
            throw new Error('userService is required for useTheme.setTheme');
        }

        await userService.updateMe({ theme: name });
        user.value.theme = name;
    }

    return {
        theme: readonly(theme),
        setTheme
    };
};