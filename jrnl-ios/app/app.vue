<template>
  <NuxtLayout>
    <NuxtPage/>
  </NuxtLayout>
</template>

<script lang="ts" setup>
import { useAllMeta } from '~/util/all-meta.util';

useAllMeta();
</script>

<style>
body, html, #__nuxt {
  height: 100%;
}
</style>