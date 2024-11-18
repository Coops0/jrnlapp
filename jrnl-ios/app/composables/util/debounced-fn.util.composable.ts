export const useDebouncedFn = (fn: () => void, delay: number = 200, maxWait: number = 2500) => {
    const timeout = ref<NodeJS.Timeout | null>(null);
    const lastExec = ref<number>(Date.now());

    const debounced = () => {
        const elapsed = Date.now() - lastExec.value;

        if (timeout.value) {
            clearTimeout(timeout.value);
        }

        if (elapsed > maxWait) {
            fn();
            lastExec.value = Date.now();
            return;
        }

        timeout.value = setTimeout(() => {
            fn();
            lastExec.value = Date.now();
        }, delay);
    };

    const cancel = () => {
        if (timeout.value) {
            clearTimeout(timeout.value);
        }
    };

    onUnmounted(cancel);

    return {
        debounced,
        cancel,
        lastExec
    };
};