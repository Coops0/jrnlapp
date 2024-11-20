export const useOnline = () => {
    const { jwt } = useAuth();
    const isOnline = ref(navigator.onLine);

    const updateOnlineStatus = () => {
        isOnline.value = navigator.onLine;
    };

    onMounted(() => {
        window.addEventListener('online', updateOnlineStatus);
        window.addEventListener('offline', updateOnlineStatus);
    });

    onUnmounted(() => {
        window.removeEventListener('online', updateOnlineStatus);
        window.removeEventListener('offline', updateOnlineStatus);
    });

    const isConnected = computed(() => jwt.value && isOnline.value);

    return { isOnline, isConnected };
};