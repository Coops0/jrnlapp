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

    <div class="mb-2 mt-4 select-none">
      <div class="flex justify-between items-center mx-auto text-sm">
        <TodayEntryLastSaved
            :last-saved="lastSaved"
            :last-saved-entry="lastSavedEntry"
            :unsaved-changes="unsavedChanges"
            :entry
        />

        <div class="flex flex-row gap-2 justify-center items-center">
          <TodayEntryTimeUntilTomorrow :tomorrow/>
          <TodayEntryEphemeralMode v-model="entry.ephemeral" :tomorrow/>
        </div>
      </div>
    </div>
    <div class="flex flex-col flex-grow">
      <ComponentErrorBoundary>
        <TodayEntryTextEditor
            v-model="entry.text"
            :initial="entry.text"
            class="flex-grow size-full"
        />

        <div class="flex-grow mt-auto mb-8 px-4 py-3 w-full select-none">
          <div class="space-y-2 mx-auto w-full">
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
import { isSameDay, parseServerDate } from '~/util/index.util';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);
const localBackendService = new LocalBackendService();

const entry = useLocalStorage(
    'entry-today',
    BLANK_ENTRY,
    value => {
      let parsed;
      try {
        parsed = JSON.parse(value);
      } catch {
        return BLANK_ENTRY();
      }

      if (!isSameDay(parseServerDate(parsed.date))) {
        return BLANK_ENTRY();
      }

      return parsed;
    }
);

onMounted(() => {
  if (!isSameDay(parseServerDate(entry.value.date))) {
    entry.value = BLANK_ENTRY();
  }
});

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