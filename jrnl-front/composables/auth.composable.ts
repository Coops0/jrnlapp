export const useAuth = () => {
    const localJwt = useCookie<string | null>('jwt');
    const jwt = useState<string | null>('jwt', () => localJwt.value);

    const logout = () => {
        jwt.value = null;
        localJwt.value = null;
    };

    watchDeep(jwt, () => {
        localJwt.value = jwt.value;
    });

    return { jwt, logout };
};