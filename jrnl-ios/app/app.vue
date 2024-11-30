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
html, body, #__nuxt {
  height: 100%;
  width: 100%;
  overflow: hidden;
}
</style>