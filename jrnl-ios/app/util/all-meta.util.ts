export const useAllMeta = () => {
    useHead({
        title: 'jr.nl',
        bodyAttrs: {
            class: 'bg-colors-primary-900'
        },
        meta: [
            {
                name: 'viewport',
                content: 'viewport-fit=cover, width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no'
            }
        ]
    });
};