<template>
  <div class="flex items-center gap-1.5">
    <h5
        v-if="show"
        class="text-colors-primary-400/80 hover:text-colors-primary-400 transition-opacity cursor-pointer flex items-center gap-1.5"
        @click="toggle"
    >
      <span v-if="savedJustNow">last saved: just now</span>
      <span v-else-if="isUnsaved">last saved: ...</span>
      <span v-else-if="lastSaved.getFullYear() === 1900">last saved: never</span>
      <span v-else>last saved: <NuxtTime :datetime="lastSaved" relative/></span>

      <span
          v-if="unsavedChanges || isUnsaved"
          class="inline-block w-1.5 h-1.5 rounded-full bg-colors-accent-400"
      />
    </h5>
    <h5
        v-else
        class="text-colors-primary-400/80 hover:text-colors-primary-400 transition-opacity cursor-pointer px-4 flex items-center gap-1.5"
        @click="toggle"
    >🎛️
    </h5>
  </div>
</template>

<script setup lang="ts">
import type { Entry } from '~/types/entry.type';
import { useLocalStorage } from '~/composables/local-storage.composable';

const props = defineProps<{
  lastSaved: Date
  lastSavedEntry: Entry | null
  entry: Entry | null
  unsavedChanges: boolean | null
}>();

const isUnsaved = ref(false);
const savedJustNow = ref(false);

const show = useLocalStorage('show-time-until', () => true);

function toggle() {
  show.value = !show.value;
}

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

  const diff = Math.abs(Date.now() - (props.lastSaved?.getTime() ?? (1000 * 2)));

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