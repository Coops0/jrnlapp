import { useLocalStorage } from '~/composables/util/local-storage.util.composable';

export const useAuth = () => {
    const jwt = useLocalStorage<string | null>('jwt', () => null);

    const logout = () => {
        jwt.value = null;
    };

    return { jwt, logout };
};