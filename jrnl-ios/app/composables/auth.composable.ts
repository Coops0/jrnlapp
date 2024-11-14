import { load } from '@tauri-apps/plugin-store';

export const useAuth = async () => {
    const cachedJwt = await load('jwt');
    const initialCachedJwt = await cachedJwt.get<string>('jwt');

    const jwt = useState<string | null>('jwt', () => initialCachedJwt || null);

    const logout = async () => {
        jwt.value = null;
        await cachedJwt.set('jwt', null);
    };

    watch(jwt, async j => {
        console.debug('saving cached jwt to cookie', j);
        await cachedJwt.set('jwt', j);
    }, { deep: true });

    return { jwt, logout };
};