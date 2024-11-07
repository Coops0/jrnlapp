import type { UserService } from '~/services/user.service';
import { useUser } from '~/composables/user.composable';

export const useTheme = (userService: UserService | null) => {
    const { user } = useUser(userService);
    const themeCookie = useCookie<string>('theme-cache', {
        priority: 'high'
    });

    const theme = useState('theme', () => user.value?.theme || 'lunar');

    const activeTheme = useColorMode();

    onMounted(() => {
        if (user.value?.theme && user.value.theme !== activeTheme.value) {
            console.debug('useTheme: user theme changed remotely, setting theme to', user.value.theme);
            theme.value = user.value.theme;
            activeTheme.value = user.value.theme;
            activeTheme.preference = user.value.theme;
        }
    });

    watch(theme, p => {
        activeTheme.value = p;
        activeTheme.preference = p;

        themeCookie.value = p;
    });

    const setThemeLocal = (name: string) => {
        theme.value = name;
        activeTheme.value = name;
        activeTheme.preference = name;
    };

    watchImmediate(user, p => {
        if (p?.theme && p.theme !== theme.value) {
            console.debug('useTheme: user theme changed, setting theme to', p.theme);
            setThemeLocal(p.theme);
        }
    }, { deep: true });

    async function setTheme(name: string) {
        if (name === theme.value) {
            return;
        }

        setThemeLocal(name);
        if (!userService) {
            throw new Error('userService is required for useTheme.setTheme');
        }

        await debouncedUpdate();
        user.value!.theme = name;
    }

    const debouncedUpdate = useDebounceFn(() => userService!.updateMe({ theme: theme.value }), 150);

    return {
        theme: readonly(theme),
        setTheme
    };
};