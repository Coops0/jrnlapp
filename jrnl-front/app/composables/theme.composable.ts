import type { UserService } from '~/services/user.service';
import { useUser } from '~/composables/user.composable';

export const useTheme = (userService: UserService | null) => {
    const { user, hasRefreshedRemotely } = useUser(userService);
    const theme = useState('theme', () => user.value?.theme || 'lunar_placeholder');

    const activeTheme = useColorMode();

    const setThemeLocal = (name: string) => {
        if (name !== activeTheme.value) {
            activeTheme.value = name;
            activeTheme.preference = name;
        }
    };

    watch(() => activeTheme.value, t => {
        if (t !== 'lunar_placeholder') {
            return;
        }

        const userTheme = user.value?.theme;
        if (userTheme) {
            console.log('Detected lunar placeholder, switched to user theme', userTheme);
            setThemeLocal(userTheme);
        } else {
            console.warn('useTheme: activeTheme is lunar_placeholder');
        }
    }, { immediate: true });

    watch(theme, setThemeLocal);

    watch(user, p => {
        if (activeTheme.value === 'lunar_placeholder' && p) {
            console.info('user watch in theme, found lunar placeholder, updating to', p.theme);
            setThemeLocal(p.theme);
        }

        if (hasRefreshedRemotely.value && p?.theme && p.theme !== theme.value) {
            console.debug('useTheme: user theme changed, setting theme to', p.theme);
            theme.value = p.theme;
            setThemeLocal(p.theme);
        }
    }, { immediate: true, deep: true });

    async function setTheme(name: string) {
        if (name === theme.value) {
            return;
        }

        if (name === 'lunar_placeholder') {
            throw new Error('Cannot set theme to lunar_placeholder');
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