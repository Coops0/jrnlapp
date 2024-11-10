<template>
  <h5
      class="text-colors-primary-400/80 hover:text-colors-primary-400 transition-opacity"
  >
    <span v-if="savedJustNow">last saved: just now</span>
    <span v-else-if="isUnsaved">last saved: ...</span>
    <span v-else-if="lastSaved.getFullYear() === 1900">last saved: never</span>
    <span v-else>last saved: <NuxtTime :datetime="lastSaved" relative/></span>
  </h5>
</template>

<script setup lang="ts">
import type { Entry } from '~/types/entry.type';

const props = defineProps<{
  lastSaved: Date;
  lastSavedEntry: Entry | null;
  entry: Entry | null;
}>();

const isUnsaved = ref(false);
const savedJustNow = ref(false);

const timeCheckInterval = ref<NodeJS.Timeout | null>(null);

onUnmounted(() => {
  if (timeCheckInterval.value) {
    clearInterval(timeCheckInterval.value);
  }
});

function check(): number {
  if (!props.entry || !props.lastSavedEntry) {
    return 0;
  }

  const diff = Math.abs(new Date().getTime() - (props.lastSaved?.getTime() ?? (1000 * 2)));

  if (JSON.stringify(props.entry) === JSON.stringify(props.lastSavedEntry)) {
    if (diff < 1200) {
      savedJustNow.value = true;
      return 2;
    }

    return 0;
  }

  if (diff < 1200) {
    savedJustNow.value = true;
    return 2;
  }

  return 1;
}

function checkAndUpdate() {
  const checked = check();
  isUnsaved.value = checked === 1;

  if (checked !== 2) {
    savedJustNow.value = false;
  }
}

watch(() => props.entry, () => checkAndUpdate(), { deep: true });

onMounted(() => {
  timeCheckInterval.value = setInterval(() => checkAndUpdate(), 250);
});
</script>
