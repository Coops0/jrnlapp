import { themes } from 'assets/themes';

export const useAllMeta = () => {
    const { theme } = useTheme(null);
    const themeColor = computed(() => themes[theme.value]?.colors?.primary[900] ?? '#111922');

    useHead({
        title: 'jr.nl',
        bodyAttrs: {
            style: {
                'background-color': 'hsl(var(--twc-colors-primary-900))'
            }
        },
        meta: [
            { name: 'description', content: 'one day at a time' },
            { charset: 'utf-8' },
            { 'http-equiv': 'X-UA-Compatible', content: 'IE=edge' },
            {
                name: 'viewport',
                content: 'width=device-width,initial-scale=1,minimum-scale=1,maximum-scale=1,user-scalable=no'
            },
            { name: 'keywords', content: 'jrnl, jr.nl, journal' },
            { name: 'theme-color', content: themeColor },
            { name: 'apple-mobile-web-app-title', content: 'jr.nl' },
            { name: 'mobile-web-app-capable', content: 'yes' },
            { name: 'apple-mobile-web-app-status-bar-style', content: 'black-transparent' },
            { name: 'application-name', content: 'jr.nl' },
            { name: 'msapplication-tooltip', content: 'one day at a time' },
            { name: 'msapplication-starturl', content: '/' }
        ],
        // https://realfavicongenerator.net
        link: [
            // { rel: 'icon', type: 'image/jpg', sizes: '16x16', href: '/icons/' },
            // { rel: 'icon', type: 'image/jpg', sizes: '32x32', href: '/favicon-32.jpg' },
            // { rel: 'icon', type: 'image/jpg', sizes: '48x48', href: '/favicon-48.jpg' },
            { rel: 'apple-touch-icon', href: '/apple-touch-icon.png' },
            { rel: 'apple-touch-icon', sizes: '76x76', href: '/apple-touch-icon.png' },
            { rel: 'apple-touch-icon', sizes: '120x120', href: '/apple-touch-icon.png' },
            { rel: 'apple-touch-icon', sizes: '152x152', href: '/apple-touch-icon.png' },
            { rel: 'apple-touch-startup-image', href: '/apple-touch-icon.png' },
            { rel: 'icon', sizes: '192x192', href: '/web-app-manifest-192x192.png' },
            { rel: 'icon', sizes: '128x128', href: '/icon-128x128.jpg' },
            { rel: 'shortcut icon', type: 'image/x-icon', href: '/favicon.icon' },
            { rel: 'apple-touch-icon-precomposed', sizes: '57x57', href: '/images/icon-52x52.jpg' },
            { rel: 'apple-touch-icon', sizes: '72x72', href: '/images/icon-72x72.jpg' }
        ]
    });
}