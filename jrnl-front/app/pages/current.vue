<template>
  <div class="flex flex-col relative flex-grow px-2 md:px-4 lg:px-10 max-w-full">
    <LazyTodayEntrySaveConflictModal
        v-if="saveConflict"
        :server="saveConflict[0]"
        :local="saveConflict[1]"
        @resolve="handleSaveConflict"
    />

    <div class="top-2 left-0 right-0 z-[2] mb-2">
      <div class="flex justify-between items-center mx-auto text-sm">
        <TodayEntryLastSaved :last-saved="lastSaved" :last-saved-entry="lastSavedEntry" :unsaved-changes="unsavedChanges" :entry/>
        <TodayEntryTimeUntilTomorrow :tomorrow/>
      </div>
    </div>
    <div class="flex flex-col flex-grow">
      <ComponentErrorBoundary>
        <TodayEntryTextEditor
            v-model="entry.text"
            :initial="entryCookie?.text"
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
import type { Entry } from '~/types/entry.type';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { theme } = useTheme(null);

const entryCookie = useCookie<Entry>('entry-today', {
  maxAge: 60 * 60 * 24
});

const {
  beginFetch,
  tomorrow,
  entry,
  lastSaved,
  lastSavedEntry,
  saveConflict,
  handleSaveConflict,
  forceSave,
  unsavedChanges
} = useTodayEntry(entryService, entryCookie);

const ratingLerpBind = (value: number) => ratingLerp(value, theme.value);
beginFetch();

onBeforeUnmount(async () => {
  if (unsavedChanges.value) {
    await forceSave();
  }
});
</script>