<template>
  <div>Waiting for login...</div>
</template>

<script setup lang="ts">
import { useTimeoutFn } from '@vueuse/core';

const user = useSupabaseUser();
const { $localApi } = useNuxtApp();

watch(user, async () => {
  if (!user.value) {
    return;
  }

  const redirectCookie = useCookie(`${useRuntimeConfig().public.supabase.cookieName}-redirect-path`);
  const path = redirectCookie?.value || '/page';

  redirectCookie.value = null;

  try {
    await $localApi('/user/tz', {
      ignoreError: true,
      method: 'PUT',
      body: JSON.stringify({ tz: Intl.DateTimeFormat().resolvedOptions().timeZone }),
    });
  } catch (e) {
    console.warn(e);
  }

  return navigateTo(path);
}, { immediate: true });

useTimeoutFn(() => navigateTo('/login'), 5000);

</script>