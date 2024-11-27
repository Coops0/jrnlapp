<template>
  <div class="relative">
    <h5
        class="text-colors-primary-400/80 hover:text-colors-primary-400 cursor-pointer flex items-center gap-1.5 transition-all duration-200 ease-in-out"
        :class="show ? 'opacity-100' : 'opacity-0'"
        @click="toggle"
    >
      <span v-if="savedJustNow">last saved: just now</span>
      <span v-else-if="isUnsaved">last saved: ...</span>
      <span v-else-if="lastSaved.getFullYear() === 1900">last saved: never</span>
      <span v-else>last saved: <NuxtTime :datetime="lastSaved" relative/></span>

      <span
          v-if="unsavedChangesBuffered || isUnsaved"
          class="inline-block w-1.5 h-1.5 rounded-full bg-colors-accent-400"
      />
    </h5>

    <div
        class="absolute top-0 left-0 right-0 bottom-0 origin-left rounded-md cursor-pointer bg-opacity-30 bg-colors-primary-400 transition-all duration-300 hover:bg-opacity-60"
        style="transition-timing-function: cubic-bezier(0.34, 1.56, 0.64, 1)"
        :class="show ? 'opacity-0' : 'opacity-100 scale-x-50'"
        @click="toggle"
    />
  </div>
</template>

<script setup lang="ts">
import type { Entry } from '~/types/entry.type';

const props = defineProps<{
  lastSaved: Date;
  lastSavedEntry: Entry | null;
  entry: Entry | null;
  unsavedChanges: boolean | null;
}>();

const unsavedChangesBuffered = ref(false);

const show = useCookie('show-last-saved', {
  default: () => true,
  maxAge: 60 * 60 * 24 * 365
});

const isUnsaved = ref(false);
const savedJustNow = ref(false);

const toggle = () => {
  show.value = !show.value;
};

const timeCheckInterval = ref<NodeJS.Timeout | null>(null);

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

watch(() => props.entry, checkAndUpdate, { deep: true });

onMounted(() => {
  // don't want indicator to show up immediately
  setTimeout(() => {
    timeCheckInterval.value = setInterval(checkAndUpdate, 250);
  }, 500);

  unsavedChangesBuffered.value = !!props.unsavedChanges;
});

onUnmounted(() => {
  if (timeCheckInterval.value) {
    clearInterval(timeCheckInterval.value);
  }
});
</script>