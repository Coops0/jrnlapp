<template>
  <div class="max-w-5xl mx-auto px-4 py-3 flex items-center justify-between">
    <NavigatorLogo :authenticated/>
    <Navigator :authenticated :name/>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  name?: string;
  authenticated: boolean;
}>();
</script>