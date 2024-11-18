<template>
  <div class="text-right">
    <h3
        v-if="show"
        class="text-colors-primary-300 hover:text-colors-primary-200 cursor-pointer text-sm"
        @click="toggle"
    >
      day ends in
      <NuxtTime :datetime="tomorrow" relative/>
    </h3>
    <h3
        v-else
        class="text-colors-primary-500 hover:text-colors-primary-300 cursor-pointer text-sm px-4"
        @click="toggle"
    >
      ☀️
    </h3>
  </div>
</template>

<script lang="ts" setup>
import { useLocalStorage } from '~/composables/local-storage.composable';

defineProps<{ tomorrow: Date }>();

const show = useLocalStorage('show-time-until', () => true);

function toggle() {
  show.value = !show.value;
}
</script>