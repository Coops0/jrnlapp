<template>
  <ErrorDisplay :error="error" @clear-error="clear">
    <FormButton size="md" variant="secondary" full @click="clearCookiesAndRefresh">
      logout & refresh
    </FormButton>
  </ErrorDisplay>
</template>

<script setup lang="ts">
import type { NuxtError } from '#app';
import { useAllMeta } from '~/util/all-meta.util';

useAllMeta();

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