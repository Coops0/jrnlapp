<template>
  <NuxtLink
      :to="authenticated ? '/page' : '/'"
      class="text-2xl font-semibold text-colors-primary-100 hover:text-colors-primary-300 transition-colors"
  >
    jrnl
  </NuxtLink>
</template>

<script lang="ts" setup>
defineProps<{ authenticated: boolean }>();
</script>