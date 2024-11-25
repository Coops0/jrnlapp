<template>
  <div class="text-right">
    <h3
        class="text-colors-primary-300 hover:text-colors-primary-200 cursor-pointer text-sm transition-opacity duration-200"
        :class="{ 'opacity-0': !show }"
        @click="toggle"
    >
      day ends in
      <NuxtTime :datetime="tomorrow" relative/>
    </h3>
    <div
        class="cursor-pointer justify-end w-1/4 border-b-2 border-solid border-colors-primary-400 opacity-60 transition-opacity duration-200"
        :class="{ 'opacity-0': show }"
        @click="toggle"
    />
  </div>
</template>

<script lang="ts" setup>
import { useLocalStorage } from '~/composables/util/local-storage.util.composable';

defineProps<{ tomorrow: Date }>();

const show = useLocalStorage('show-time-until', () => true);

function toggle() {
  show.value = !show.value;
}
</script>