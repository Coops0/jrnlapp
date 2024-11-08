import type { UserService } from '~/services/user.service';
import { useUser } from '~/composables/user.composable';

export const useTheme = (userService: UserService | null) => {
    const { user } = useUser(userService);
    const theme = useState('theme', () => user.value?.theme || 'lunar');
    const themeCacheCookie = useCookie('theme-cache', {
        watch: false,
        path: '',
        priority: 'high'
    });

    const activeTheme = useColorMode();

    const setThemeLocal = (name: string) => {
        if (name !== activeTheme.value) {
            themeCacheCookie.value = name;

            activeTheme.forced = true;
            // activeTheme.value = name;
            activeTheme.preference = name;
        }
    };

    onMounted(() => {
        if (user.value?.theme && user.value.theme !== activeTheme.value) {
            console.debug('useTheme: user theme changed remotely, setting theme to', user.value.theme);
            theme.value = user.value.theme;
            setThemeLocal(user.value.theme);
        }
    });

    watch(theme, setThemeLocal);

    watch(user, p => {
        if (p?.theme && p.theme !== theme.value) {
            console.debug('useTheme: user theme changed, setting theme to', p.theme);
            theme.value = p.theme;
            setThemeLocal(p.theme);
        }
    }, { immediate: true, deep: true });

    async function setTheme(name: string) {
        if (name === theme.value) {
            return;
        }

        theme.value = name;
        setThemeLocal(name);
        if (!userService) {
            throw new Error('userService is required for useTheme.setTheme');
        }

        await userService!.updateMe({ theme: theme.value });
        user.value!.theme = name;
    }

    return {
        theme: readonly(theme),
        setTheme
    };
};