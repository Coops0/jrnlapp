import type { UserService } from '~/services/user.service';
import { useUser } from '~/composables/user.composable';
import { load } from '@tauri-apps/plugin-store';

export const useTheme = async (userService: UserService | null) => {
    const { user, hasRefreshedRemotely } = await useUser(userService);

    const themeStore = await load('theme.json');
    const initialCachedTheme = await themeStore.get<string>('theme');

    const theme = useState('theme', () => user.value?.theme || initialCachedTheme || 'lunar_placeholder');

    watch(theme, async t => {
        document.body.setAttribute('data-theme', t);
        await themeStore.set('theme', t);
    }, { immediate: true });

    watch(user, p => {
        if (theme.value === 'lunar_placeholder' && p) {
            theme.value = p.theme;
        }

        if (hasRefreshedRemotely.value && p?.theme && p.theme !== theme.value) {
            console.debug('useTheme: user theme changed, setting theme to', p.theme);
            theme.value = p.theme;
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