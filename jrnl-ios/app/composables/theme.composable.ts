import type { UserService } from '~/services/user.service';
import { useUser } from '~/composables/user.composable';
import { useLocalStorage } from '~/composables/local-storage.composable';

export const useTheme = (userService: UserService | null) => {
    const { user, hasRefreshedRemotely } = useUser(userService);

    const theme = useLocalStorage<string>('theme', () => user.value?.theme || 'lunar_placeholder');

    watch(theme, async t => {
        document.body.setAttribute('data-theme', t);
    }, { immediate: true });

    watch(user, p => {
        if (theme.value === 'lunar_placeholder' && p) {
            theme.value = p.theme;
        }

        if (hasRefreshedRemotely.value && p?.theme && p.theme !== theme.value) {
            console.debug('useTheme: user theme changed, setting theme to', p.theme);
            theme.value = p.theme;
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
        if (userService) {
            await userService!.updateMe({ theme: theme.value });
            user.value!.theme = name;
        }
    }

    return {
        theme: readonly(theme),
        setTheme
    };
};