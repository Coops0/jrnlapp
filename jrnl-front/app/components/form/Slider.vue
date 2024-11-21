<template>
  <div class="relative w-full h-12 flex items-center">
    <div class="absolute h-[2px] w-full bg-colors-primary-700/50 rounded-full overflow-hidden">
      <div
          :style="{
             width: percentage + '%',
             backgroundColor: ratingLerp(modelValue ?? 0, theme)
        }"
          class="absolute h-full transition-all ease-out duration-100"
      />
    </div>

    <div
        :style="{ left: 'calc(' + percentage + '% - 8px)' }"
        class="absolute w-4 h-4 rounded-full bg-colors-primary-200 hover:bg-colors-primary-100 transition-colors shadow-sm pointer-events-none"
    />

    <input
        v-model.number="value"
        :max="max"
        :min="min"
        :step="step"
        class="absolute w-full h-full opacity-0 cursor-pointer"
        type="range"
    >
  </div>
</template>

<script lang="ts" setup>
import { ratingLerp } from '~/util/index.util';

const { theme } = useTheme(null);

const value = defineModel<number>({ default: 0 });

const props = defineProps<{
  min?: number;
  max?: number;
  step?: number;
}>();

const percentage = computed(() => {
  const range = (props.max ?? 100) - (props.min ?? 0);
  const ratio = ((value.value ?? 5) - (props.min ?? 0)) / range;

  return ratio * 100;
});
</script>