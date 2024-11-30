export const useLocalStorage = <T>(
    name: string,
    defaultValue: () => T,
    parseFromStorage: (value: string) => T = JSON.parse
) => {
    const fetch = () => {
        const item = localStorage.getItem(name);
        return item ? parseFromStorage(item) : defaultValue?.();
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