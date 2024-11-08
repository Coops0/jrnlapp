<template>
  <h5
      class="text-colors-primary-400/80 hover:text-colors-primary-400 transition-opacity"
  >
    <span v-if="isSaving">last saved: ...</span>
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

const isSaving = ref(false);
const timeCheckInterval = ref<NodeJS.Timeout | null>(null);

watch(() => props.entry, e => {
  if (!props.lastSavedEntry || JSON.stringify(props.lastSavedEntry) !== JSON.stringify(e)) {
    isSaving.value = true;
  } else {
    isSaving.value = !isTimeTooClose();
  }
}, { deep: true });

onMounted(() => {
  timeCheckInterval.value = setInterval(() => {
    isSaving.value = !isTimeTooClose();
  }, 1000);
});

onUnmounted(() => {
  if (timeCheckInterval.value) {
    clearInterval(timeCheckInterval.value);
  }
});

function isTimeTooClose(): boolean {
  const diff = Math.abs(new Date().getTime() - (props.lastSaved?.getTime() ?? 0));
  return diff < 1000 * 60;
}

</script>
