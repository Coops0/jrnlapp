export const useAuth = () => {
    const cachedJwt = useCookie<string | null>('jwt', {
        default() {
            return null;
        }
    });

    const jwt = useState<string | null>('jwt', () => cachedJwt.value);

    const logout = () => {
        jwt.value = null;
        cachedJwt.value = null;
    };

    watchDeep(jwt, j => {
        console.debug('saving cached jwt to cookie', j);
        cachedJwt.value = j;
    });

    return { jwt, logout };
};