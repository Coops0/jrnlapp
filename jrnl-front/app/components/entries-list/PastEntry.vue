<template>
  <NuxtLink
      :disabled="loading"
      :style="{ 'background-color': color }"
      :to="loading ? '' : { name: 'past-id', params: { id } }"
      prefetch
      :prefetch-on="{ interaction: true }"
      class="block w-full rounded-lg transition-all duration-200 hover:scale-[1.01] group relative overflow-hidden"
      :class="{ 'cursor-default': loading }"
  >
    <div
        v-if="loading"
        class="absolute inset-0 animate-pulse bg-colors-primary-50/5"
    />

    <div class="flex items-center justify-between px-3 py-2 relative">
      <span class="text-colors-primary-50 text-sm">
        <span v-if="loading" class="animate-pulse">loading...</span>
        <NuxtTime v-else :datetime="date" day="numeric" month="short"/>
      </span>
      <span
          class="text-colors-primary-50/80 text-xs"
          :class="{ 'opacity-0': loading }"
      >
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
  loading?: boolean;
}>();
</script>