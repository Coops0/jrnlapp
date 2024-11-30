<template>
  <div
      ref="containerRef"
      class="relative w-full h-16 flex items-center select-none"
      @mousedown="startDragging"
      @touchstart="startDragging"
  >
    <!-- Track -->
    <div
        class="absolute h-2 w-full rounded-full overflow-hidden"
        :class="[
          isDragging ? 'bg-colors-primary-600/50' : 'bg-colors-primary-700/50',
          !isDragging && 'transition-colors duration-200'
        ]"
    >
      <!-- Fill -->
      <div
          :style="{
            width: percentage + '%',
            backgroundColor: ratingLerp(modelValue ?? 0, theme)
          }"
          :class="!isDragging && 'transition-all duration-200 ease-out'"
          class="absolute h-full transform-gpu will-change-[width]"
      />
    </div>

    <!-- Thumb -->
    <div
        :style="{
          left: `calc(${percentage}% - 12px)`,
          backgroundColor: ratingLerp(modelValue ?? 0, theme),
          transform: isDragging ? 'scale(1.1)' : 'scale(1)'
        }"
        :class="!isDragging && 'transition-all duration-200 ease-out'"
        class="absolute w-5 h-5 rounded-full shadow-md pointer-events-none touch-none transform-gpu will-change-[left,transform]"
    />

    <input
        ref="inputRef"
        v-model.number="value"
        :max="max"
        :min="min"
        :step="step"
        type="range"
        class="absolute w-full h-16 opacity-0 cursor-grab active:cursor-grabbing touch-none [-webkit-tap-highlight-color:transparent]"
    >
  </div>
</template>

<script lang="ts" setup>
import { ratingLerp } from '~/util/index.util';

const { theme } = useTheme(null);
const value = defineModel<number>({ default: 0 });
const inputRef = ref<HTMLInputElement | null>(null);
const containerRef = ref<HTMLElement | null>(null);
const isDragging = ref(false);

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

function updateValue(clientX: number) {
  if (!containerRef.value) {
    return;
  }

  const rect = containerRef.value.getBoundingClientRect();
  const percentage = (clientX - rect.left) / rect.width;
  const range = (props.max ?? 100) - (props.min ?? 0);

  let newValue = (props.min ?? 0) + (percentage * range);

  newValue = Math.min(props.max ?? 100, Math.max(props.min ?? 0, newValue));

  if (props.step) {
    newValue = Math.round(newValue / props.step) * props.step;
  }

  value.value = newValue;
}

function onMove(event: MouseEvent | TouchEvent) {
  if (!isDragging.value) {
    return;
  }

  event.preventDefault();

  const clientX = event instanceof MouseEvent
      ? event.clientX
      : event.touches?.[0]?.clientX;

  updateValue(clientX ?? 0);
}

function onEnd() {
  isDragging.value = false;
  window.removeEventListener('mousemove', onMove);
  window.removeEventListener('mouseup', onEnd);
  window.removeEventListener('touchmove', onMove);
  window.removeEventListener('touchend', onEnd);
  window.removeEventListener('touchcancel', onEnd);
}

function startDragging(event: MouseEvent | TouchEvent) {
  isDragging.value = true;

  const clientX = event instanceof MouseEvent
      ? event.clientX
      : event.touches?.[0]?.clientX;

  updateValue(clientX ?? 0);

  window.addEventListener('mousemove', onMove, { passive: false });
  window.addEventListener('mouseup', onEnd);
  window.addEventListener('touchmove', onMove, { passive: false });
  window.addEventListener('touchend', onEnd);
  window.addEventListener('touchcancel', onEnd);
}

onBeforeUnmount(() => onEnd());
</script>

<style scoped>
input[type="range"] {
  -webkit-appearance: none;
  background: transparent;
}

@media (pointer: coarse) {
  input[type="range"] {
    padding: 0;
    margin: 0;
  }
}
</style>