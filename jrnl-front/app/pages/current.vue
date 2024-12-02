<template>
  <div class="flex flex-col relative flex-grow px-2 md:px-4 lg:px-10 max-w-full">
    <LazyTodayEntryNewDayAnimation
        :show="showDayTransition"
        @transition-complete="handleTransitionComplete"
    />

    <LazyTodayEntrySaveConflictModal
        v-if="saveConflict"
        :server="saveConflict[0]"
        :local="saveConflict[1]"
        @resolve="handleSaveConflict"
    />

    <LazyOnboardingTour v-if="tourEnabled"/>

    <div class="top-2 left-0 right-0 z-[2] mb-2 select-none">
      <div class="flex justify-between items-center mx-auto text-sm">
        <TodayEntryLastSaved
            id="last-saved"
            :last-saved="lastSaved"
            :last-saved-entry="lastSavedEntry"
            :unsaved-changes="unsavedChanges"
            :entry
        />

        <div class="flex flex-row gap-2 justify-center items-center">
          <TodayEntryTimeUntilTomorrow id="tomorrow-lock" :tomorrow/>
          <TodayEntryEphemeralMode id="ephemeral-mode" v-model="entry.ephemeral" :tomorrow/>
        </div>
      </div>
    </div>
    <div class="flex flex-col flex-grow">
      <ComponentErrorBoundary>
        <TodayEntryTextEditor
            v-model="entry.text"
            :initial="entryCookie?.text"
            class="flex-grow h-full w-full"
        />
        <div class="flex-grow mt-auto px-4 py-3 select-none">
          <div class="space-y-2 mx-auto">
            <FormSlider
                id="emotion-slider"
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
import type { Entry } from '~/types/entry.type';
import { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);
const userService = new UserService($localApi);

const { user } = useUser(userService);

const entryCookie = useCookie<Entry>('entry-today', {
  maxAge: 60 * 60 * 24 * 30
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
  unsavedChanges,
  showDayTransition
} = useTodayEntry(entryService, entryCookie);
beginFetch();

watch(user, u => checkAndRunTour(u), { immediate: true, deep: true });

const tourEnabled = ref(false);

async function checkAndRunTour(u: User | null) {
  if (u?.has_had_tour !== false) {
    return;
  }

  user.value = await userService.updateMe({ has_had_tour: true });
  tourEnabled.value = true;
}

onBeforeUnmount(() => {
  if (unsavedChanges.value) {
    forceSave();
  }
});

const handleTransitionComplete = () => {
  showDayTransition.value = false;
};
</script>