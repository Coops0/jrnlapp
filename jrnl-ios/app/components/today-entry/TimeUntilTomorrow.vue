<template>
  <div class="text-right relative">
    <div class="relative inline-block">
      <h3
          class="text-colors-primary-300 hover:text-colors-primary-200 cursor-pointer text-sm transition-all duration-200 ease-in-out"
          :class="show ? 'opacity-100' : 'opacity-0'"
          @click="toggle"
      >
        day ends in
        <NuxtTime :datetime="tomorrow" relative/>
      </h3>
      <div
          class="absolute top-0 left-0 right-0 bottom-0 origin-right rounded-lg cursor-pointer bg-opacity-30 bg-colors-primary-400 transition-[transform,opacity] duration-300"
          style="transition-timing-function: cubic-bezier(0.34, 1.56, 0.64, 1)"
          :class="show ? 'opacity-0' : 'opacity-100 scale-x-50'"
          @click="toggle"
      />
    </div>
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