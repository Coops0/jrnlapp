<template>
  <div class="transition-all duration-250 opacity-50 hover:opacity-80">
    <div class="max-w-5xl mx-auto px-4 py-3 flex items-center justify-between">
      <NavigatorLogo/>
      <NavigatorLinks :name/>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ name?: string }>();
</script>