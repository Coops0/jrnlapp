<template>
  <div class="text-colors-text-200">redirecting</div>
</template>

<script lang="ts" setup>
const { public: { apiBase } } = useRuntimeConfig();

definePageMeta({ redirectUnautheticated: false });

await navigateTo(`${apiBase}/auth/google`, { external: true });
</script>