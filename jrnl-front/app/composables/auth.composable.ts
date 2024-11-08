export const useAuth = () => {
    const cachedJwt = useCookie<string | null>('jwt', { default: () => null });

    const jwt = useState<string | null>('jwt', () => cachedJwt.value);

    const logout = () => {
        jwt.value = null;
        cachedJwt.value = null;
    };

    watch(jwt, j => {
        console.debug('saving cached jwt to cookie', j);
        cachedJwt.value = j;
    }, { deep: true });

    return { jwt, logout };
};