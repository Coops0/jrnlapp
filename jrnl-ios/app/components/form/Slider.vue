<template>
  <div class="relative w-full h-16 flex items-center select-none">
    <div class="absolute h-2 w-full bg-colors-primary-700/50 rounded-full overflow-hidden">
      <div
          :style="{
             width: percentage + '%',
             backgroundColor: ratingLerp(modelValue ?? 0, theme)
        }"
          class="absolute h-full transition-all ease-out duration-0"
      />
    </div>

    <div
        class="absolute w-5 h-5 transition-all duration-0 rounded-full bg-colors-primary-200 hover:bg-colors-primary-100 shadow-md pointer-events-none touch-none"
        :style="{
            left: `calc(${percentage}% - 12px)`,
            backgroundColor: ratingLerp(modelValue ?? 0, theme)
        }"
    />

    <input
        v-model.number="value"
        :max="max"
        :min="min"
        :step="step"
        class="absolute w-full h-16 opacity-0 cursor-pointer touch-none tap-highlight-none"
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