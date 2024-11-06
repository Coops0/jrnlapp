<template>
  <NuxtLink
      :to="{ name: 'entry-id', params: { id } }"
      class="w-64 h-64 rounded-xl"
      :style="{'background-color': ratingLerp(rating, theme)}"
  >
    <p class="flex items-center justify-center h-full">{{ date }}</p>
  </NuxtLink>
</template>

<script lang="ts" setup>
import { ratingLerp } from '~/util/index.util';

const { theme } = useTheme(null);

defineProps<{
  id: string;
  rating: number;
  date: string;
}>();

</script>