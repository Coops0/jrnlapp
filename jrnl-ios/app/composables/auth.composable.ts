import { useLocalStorage } from '~/composables/local-storage.composable';

export const useAuth = () => {
    const jwt = useLocalStorage<string | null>('jwt', () => null);

    const logout = () => {
        jwt.value = null;
    };

    return { jwt, logout };
};