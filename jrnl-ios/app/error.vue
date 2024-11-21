<template>
  <ErrorDisplay class="size-full" :error="error" @clear-error="clear" />
</template>

<script setup lang="ts">
import type { NuxtError } from '#app';

const props = defineProps<{ error: NuxtError }>();

onMounted(() => {
  console.debug(props.error.stack);
});

function clearCookiesAndRefresh() {
  useCookie('theme-cache').value = null;
  useCookie('jwt').value = null;
  useCookie('cached-user').value = null;
  useCookie('entry-today').value = null;


  clearNuxtData();
  clearNuxtState();

  reloadNuxtApp({ path: '/', force: true });
}

const clear = () => clearError({ redirect: '/' });
</script>

<style>
/* noinspection CssUnusedSymbol */
body, html, #__nuxt {
  width: 100%;
  height: 100%
}
</style>