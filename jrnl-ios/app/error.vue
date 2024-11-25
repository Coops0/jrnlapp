<template>
  <ErrorDisplay :error="error" @clear-error="clear">
    <FormButton size="md" variant="secondary" full @click="clearLocalStorageAndRefresh">
      logout & refresh app
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

function clearLocalStorageAndRefresh() {
  localStorage.removeItem('theme-cache');
  localStorage.removeItem('jwt');
  localStorage.removeItem('cached-user');
  localStorage.removeItem('entry-today');


  clearNuxtData();
  clearNuxtState();

  reloadNuxtApp({ path: '/', force: true });
}

const clear = () => clearError({ redirect: '/' });
</script>