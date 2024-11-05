<template>
  <div>
    <NuxtLink :to="{ name: 'entry-id', params: { id } }">
      <div :style="{ 'background-color': ratingLerp(rating) }" class="w-5 h-5 rounded-full">
      </div>
    </NuxtLink>
  </div>
</template>

<script lang="ts" setup>
import { ratingLerp } from '~/util/index.util';

const props = defineProps<{
  id: string;
  rating: number;
  date: string;
}>();

// color based on rating


</script>