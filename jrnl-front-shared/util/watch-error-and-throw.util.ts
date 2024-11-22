export const watchErrorAndThrow = (error: Ref<Error | undefined>) => {
    watch(error, value => {
        if (value) {
            throw value;
        }
    }, { deep: true, immediate: true });
};