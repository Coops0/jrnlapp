<template>
  <NuxtLayout>
    <NuxtPage/>
  </NuxtLayout>
</template>

<script lang="ts" setup>
import { themes } from 'assets/themes';

const { theme } = useTheme(null);

useHead({
  title: 'jr.nl',
  meta: [
    { name: 'description', content: 'you can\'t cheat your thoughts' },
    { charset: 'utf-8' },
    { 'http-equiv': 'X-UA-Compatible', content: 'IE=edge' },
    {
      name: 'viewport',
      content: 'width=device-width,initial-scale=1,minimum-scale=1,maximum-scale=1,user-scalable=no'
    },
    { name: 'keywords', content: 'jrnl, jr.nl, journal' },
    { name: 'theme-color', content: themes[theme.value]?.colors?.primary[900] ?? '#111922' },
    { name: 'apple-mobile-web-app-title', content: 'jr.nl' },
    { name: 'mobile-web-app-capable', content: 'yes' },
    { name: 'apple-mobile-web-app-status-bar-style', content: 'black-transparent' },
    { name: 'application-name', content: 'jr.nl' },
    { name: 'msapplication-tooltip', content: 'you can\'t cheat your thoughts' },
    { name: 'msapplication-starturl', content: '/' }
  ],
  // https://realfavicongenerator.net
  // npx realfavicon generate
  // link: [
  //   { rel: 'icon', type: 'image/jpg', sizes: '16x16', href: '/favicon-16.jpg' },
  //   { rel: 'icon', type: 'image/jpg', sizes: '32x32', href: '/favicon-32.jpg' },
  //   { rel: 'icon', type: 'image/jpg', sizes: '48x48', href: '/favicon-48.jpg' },
  //   { rel: 'apple-touch-icon', href: '/touch-icon-iphone.jpg' },
  //   { rel: 'apple-touch-icon', sizes: '76x76', href: '/touch-icon-ipad.jpg' },
  //   { rel: 'apple-touch-icon', sizes: '120x120', href: '/touch-icon-iphone-retina.jpg' },
  //   { rel: 'apple-touch-icon', sizes: '152x152', href: '/touch-icon-ipad-retina.jpg' },
  //   { rel: 'apple-touch-startup-image', href: '/touch-icon-start-up-320x480.jpg' },
  //   { rel: 'icon', sizes: '192x192', href: '/icon-192x192.jpg' },
  //   { rel: 'icon', sizes: '128x128', href: '/icon-128x128.jpg' },
  //   { rel: 'shortcut icon', type: 'image/x-icon', href: '/favicon.icon' },
  //   { rel: 'apple-touch-icon-precomposed', sizes: '57x57', href: '/images/icon-52x52.jpg' },
  //   { rel: 'apple-touch-icon', sizes: '72x72', href: '/images/icon-72x72.jpg' }
  // ]
});
</script>