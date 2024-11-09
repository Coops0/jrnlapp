<template>
  <NuxtLink
      :disabled
      :style="{ 'background-color': color }"
      :to="disabled ? '' : { name: 'past-id', params: { id } }"
      prefetch
      :prefetch-on="{ interaction: true }"
      class="block w-full rounded-lg transition-transform duration-200 hover:scale-[1.01] group"
  >
    <div class="flex items-center justify-between px-3 py-2">
      <span class="text-colors-primary-50 text-sm">
        <span v-if="disabled">loading...</span>
        <NuxtTime :datetime="date" day="numeric" month="short"/>
      </span>
      <span class="text-colors-primary-50/80 text-xs">
        {{ rating.toFixed(1) }}
      </span>
    </div>
  </NuxtLink>
</template>

<script lang="ts" setup>
defineProps<{
  id: string;
  date: Date;
  rating: number;
  color: string;
  disabled?: boolean;
}>();
</script>