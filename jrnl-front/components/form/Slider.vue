<template>
  <div class="relative w-full h-12 flex items-center">
    <div class="absolute h-[2px] w-full bg-colors-primary-700/50 rounded-full overflow-hidden">
      <div
          class="absolute h-full transition-all duration-100"
          :style="{
          width: `${percentage}%`,
          backgroundColor: ratingLerp(modelValue ?? 0)
        }"
      />
    </div>

    <div
        class="absolute w-4 h-4 rounded-full bg-colors-primary-200 hover:bg-colors-primary-100 transition-colors shadow-sm pointer-events-none"
        :style="{ left: `calc(${percentage}% - 8px)` }"
    />

    <input
        type="range"
        v-model.number="value"
        :min="min"
        :max="max"
        :step="step"
        class="absolute w-full h-full opacity-0 cursor-pointer"
    />
  </div>
</template>

<script lang="ts" setup>
const value = defineModel<number>({default: 0});

const props = defineProps<{
  min?: number;
  max?: number;
  step?: number;
  ratingLerp: (value: number) => string;
}>();

const percentage = computed(() => {
  const range = (props.max ?? 100) - (props.min ?? 0);
  const ratio = ((value.value ?? 5) - (props.min ?? 0)) / range;

  return ratio * 100;
});
</script>