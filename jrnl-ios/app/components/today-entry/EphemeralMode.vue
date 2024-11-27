<template>
  <div>
    <TodayEntryEphemeralDialog v-model="shouldShowDialog" :tomorrow @confirm="handleConfirmation"/>
    <div class="relative">
      <button
          class="p-2 rounded-lg transition-colors duration-300 active:bg-colors-primary-600/20 focus:outline-none flex items-center gap-2"
          :class="ephemeral ? 'text-colors-accent-400 bg-colors-primary-500/30' : 'text-colors-primary-200'"
          @click="handleButtonClick"
      >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-4 w-4"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
        >
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 8v4"/>
          <path d="M12 16h.01"/>
        </svg>
      </button>

      <div v-if="showPulse" class="fixed inset-0 pointer-events-none">
        <div
            v-for="(blotch, i) in blotches"
            :key="i"
            class="absolute bg-colors-accent-400/10 rounded-full transition-all duration-1000"
            :style="{
          width: `${blotch.size}px`,
          height: `${blotch.size}px`,
          left: `${blotch.x}%`,
          top: `${blotch.y}%`,
          transform: `scale(${blotch.scale})`,
          opacity: blotch.opacity,
          transitionDuration: `${blotch.duration}ms`
        }"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
const ephemeral = defineModel<boolean>({ required: true, default: false });
defineProps<{ tomorrow: Date }>();

const shouldShowDialog = ref(false);

const showPulse = ref(false);
const blotches = ref<Array<{
  x: number;
  y: number;
  size: number;
  scale: number;
  opacity: number;
  duration: number;
}>>([]);

const createBlotches = () => {
  const amount = Math.random() * 10;
  blotches.value = Array.from({ length: amount }, () => ({
    x: Math.random() * 100,
    y: Math.random() * 100,
    size: 100 + Math.random() * 200,
    scale: Math.max(0, 0.5 - Math.random()),
    opacity: Math.random(),
    duration: 600 + Math.random() * 2000,
  }));

  requestAnimationFrame(() => {
    blotches.value = blotches.value.map(blotch => ({
      ...blotch,
      scale: 1,
      opacity: 0
    }));
  });
};


const handleConfirmation = () => {
  shouldShowDialog.value = false;

  ephemeral.value = true;
  if (showPulse.value) {
    return;
  }

  createBlotches();
  showPulse.value = true;

  setTimeout(() => {
    showPulse.value = false;
  }, 2600);
};

const handleButtonClick = () => {
  if (ephemeral.value) {
    ephemeral.value = false;
  } else {
    shouldShowDialog.value = true;
  }
};
</script>