<template>
  <NuxtLink
      :disabled
      :prefetch="true"
      :to="disabled ? '' : { name: 'groups-code', params: { code } }"
      class="block p-4 rounded-lg bg-colors-primary-800/40 hover:bg-colors-primary-700/60 transition-colors group"
  >
    <div class="flex items-center justify-between">
      <h3 class="text-colors-primary-200 group-hover:text-colors-primary-100 transition-colors">
        {{ name }}
      </h3>
      <span class="text-xs text-colors-primary-500">
        {{ code }}
      </span>
    </div>
  </NuxtLink>
</template>

<script lang="ts" setup>
defineProps<{
  name: string;
  code: string;
  disabled?: boolean;
}>();
</script>