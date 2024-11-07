<template>
  <NuxtLink
      :disabled
      :style="{ 'background-color': color }"
      :to="disabled ? '' : { name: 'entry-id', params: { id } }"
      class="block w-full rounded-lg transition-transform duration-200 hover:scale-[1.01] group"
  >
    <div class="flex items-center justify-between px-3 py-2">
      <span class="text-colors-primary-50 text-sm">
        <span v-if="disabled">loading...</span>
        <span v-else data-allow-mismatch="text">{{
            parsedDate.toLocaleDateString('en-US', {
              month: 'short',
              day: 'numeric'
            })
          }}</span>
      </span>
      <span class="text-colors-primary-50/80 text-xs">
        {{ rating.toFixed(1) }}
      </span>
    </div>
  </NuxtLink>
</template>

<script lang="ts" setup>
import { parseServerDate } from '~/util/index.util';

const props = defineProps<{
  id: string;
  date: string;
  rating: number;
  color: string;
  disabled?: boolean;
}>();

const parsedDate = computed(() => parseServerDate(props.date));
</script>