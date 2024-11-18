<template>
  <div class="flex flex-col relative flex-grow px-2 md:px-4 lg:px-10 max-w-full">
    <LazyTodayEntrySaveConflictModal
        v-if="saveConflict"
        :server="saveConflict[0]"
        :local="saveConflict[1]"
        @resolve="handleSaveConflict"
    />

    <div class="z-[2] mb-2 mt-4">
      <div class="flex justify-between items-center mx-auto text-sm">
        <TodayEntryLastSaved
            :last-saved="lastSaved"
            :last-saved-entry="lastSavedEntry"
            :unsaved-changes="unsavedChanges"
            :entry
        />
        <TodayEntryTimeUntilTomorrow :tomorrow/>
      </div>
    </div>
    <div class="flex flex-col flex-grow">
      <ComponentErrorBoundary>
        <TodayEntryTextEditor
            v-model="entry.text"
            :initial="null"
            class="flex-grow h-full w-full"
        />
        <div class="flex-grow mt-auto px-4 py-3">
          <div class="space-y-2 mx-auto">
            <FormSlider
                v-model="entry.emotion_scale"
                :max="10"
                :min="0"
                :rating-lerp="ratingLerpBind"
                :step="0.1"
            />
          </div>
        </div>
      </ComponentErrorBoundary>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { ratingLerp } from '~/util/index.util';
import { BLANK_ENTRY } from '~/composables/local-today-entry.composable';
import { useLocalStorage } from '~/composables/local-storage.composable';

const emit = defineEmits<{
  forceLocal: []
}>();

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { theme } = useTheme(null);

const entry = useLocalStorage('entry-today', BLANK_ENTRY);

const {
  fetchToday,
  tomorrow,
  lastSaved,
  lastSavedEntry,
  saveConflict,
  handleSaveConflict,
  forceSave,
  unsavedChanges,
  mounted,
  unMounted
} = useRemoteTodayEntry(entryService, entry);

const ratingLerpBind = (value: number) => ratingLerp(value, theme.value);
fetchToday();

onMounted(mounted);
onUnmounted(unMounted);

onBeforeUnmount(async () => {
  if (unsavedChanges.value) {
    await forceSave();
  }
});

onErrorCaptured(() => {
  try {
    forceSave();
  } catch {
    /* empty */
  }

  emit('forceLocal');
});
// watch(error, async e => {
//   if (e) {
//     try {
//       await forceSave();
//     } catch {
//       /* empty */
//     }
//
//     emit('forceLocal');
//   }
// });
</script>