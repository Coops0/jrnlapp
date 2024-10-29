<template>
  <div>
    <NuxtLink :to="{ name: 'entry-id', params: { id } }">
      <div class="w-5 h-5 rounded-full" :style="{ 'background-color': ratingLerp(rating) }">
      </div>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
import { ratingLerp } from '~/util/index.util';

const props = defineProps<{
  id: string;
  rating: number;
  date: string;
}>();

// color based on rating


</script>