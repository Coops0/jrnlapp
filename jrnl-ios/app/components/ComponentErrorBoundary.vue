<template>
  <NuxtErrorBoundary>
    <slot/>
    <template #error="{ error, clearError }">
      <ErrorDisplay :error="error" @clear-error="clearError"/>
    </template>
  </NuxtErrorBoundary>
</template>

<script setup lang="ts">
import ErrorDisplay from '~/components/ErrorDisplay.vue';
</script>