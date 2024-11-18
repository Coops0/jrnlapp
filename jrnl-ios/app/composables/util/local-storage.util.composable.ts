export const useLocalStorage = <T>(name: string, defaultValue?: () => T) => {
    const fetch = () => {
        const item = localStorage.getItem(name);
        return item ? JSON.parse(item) : defaultValue?.();
    };

    const value = useState<T>(name, fetch);

    return customRef((track, trigger) => ({
        get() {
            track();
            return value.value;
        },
        set(newValue) {
            value.value = newValue;
            localStorage.setItem(name, JSON.stringify(newValue));

            trigger();
        }
    }));
};