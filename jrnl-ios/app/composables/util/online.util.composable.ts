export const useOnline = () => {
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

    return { isOnline };
};