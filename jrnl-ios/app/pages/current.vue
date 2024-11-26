<template>
  <div class="flex flex-col relative flex-grow px-2 md:px-4 lg:px-10 max-w-full">
    <TodayEntryNewDayAnimation
        :show="showDayTransition"
        @transition-complete="handleTransitionComplete"
    />

    <LazyTodayEntrySaveConflictModal
        v-if="saveConflict"
        :server="saveConflict[0]"
        :local="saveConflict[1]"
        @resolve="handleSaveConflict"
    />

    <TodayEntryEphemeralMode
        v-model="entry.ephemeral"
        :tomorrow
        class="absolute top-2 right-2"
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
            :initial="entry.text"
            class="flex-grow h-full w-full"
        />
        <div class="flex-grow mt-auto px-4 py-3">
          <div class="space-y-2 mx-auto">
            <FormSlider
                v-model="entry.emotion_scale"
                :max="10"
                :min="0"
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
import { useLocalStorage } from '~/composables/util/local-storage.util.composable';
import { LocalBackendService } from '~/services/local-backend.service';
import { BLANK_ENTRY, useTodayEntry } from '~/composables/today-entry.composable';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);
const localBackendService = new LocalBackendService();

const entry = useLocalStorage('entry-today', BLANK_ENTRY);

const {
  fetchToday,
  tomorrow,
  lastSaved,
  lastSavedEntry,
  saveConflict,
  handleSaveConflict,
  saveNow,
  unsavedChanges,
  showDayTransition
} = useTodayEntry(entry, localBackendService, entryService);

fetchToday();

onErrorCaptured(() => {
  try {
    saveNow();
  } catch {
    /* empty */
  }
});

const handleTransitionComplete = () => {
  showDayTransition.value = false;
};
</script>