<template>
  <NuxtLayout>
    <NuxtPage/>
  </NuxtLayout>
</template>

<script lang="ts" setup>
import { useAllMeta } from '~/util/all-meta.util';

void useTheme(null);
useAllMeta();
</script>

<style>
html, body, #__nuxt {
  max-height: 100vh;
  height: 100%;
  width: 100%;
}

body {
  margin-top: env(safe-area-inset-top);
}
</style>