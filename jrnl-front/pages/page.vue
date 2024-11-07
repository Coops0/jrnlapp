<template>
  <div class="flex flex-col relative flex-grow">
    <div class="top-2 left-0 right-0 px-4 z-10 mb-2">
      <div class="flex justify-between items-center max-w-5xl mx-auto text-sm">
        <h5
            class="text-colors-primary-400/80 hover:text-colors-primary-400 transition-opacity"
            data-allow-mismatch
        >
          <span v-if="lastSaved.getFullYear() === 1900">last saved: never</span>
          <span v-else-if="lastSavedRelativeString.includes('in')">last saved ...</span>
          <span v-else>last saved {{ lastSavedRelativeString }}</span>
        </h5>
        <TodayEntryTimeUntilTomorrow :tomorrow="tomorrow"/>
      </div>
    </div>
    <div class="flex flex-col flex-grow">
      <TodayEntryTextEditor
          v-model="entry.text"
          class="flex-grow h-full w-full"
      />

      <div class="flex-grow mt-auto px-4 py-3 bg-colors-primary-900/50">
        <div class="space-y-2 max-w-5xl mx-auto">
          <FormSlider
              v-model="entry.emotion_scale"
              :max="10"
              :min="0"
              :rating-lerp="ratingLerpBind"
              :step="0.1"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { ratingLerp } from '~/util/index.util';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { beginFetch, tomorrow, entry, lastSaved } = useTodayEntry(entryService);
const lastSavedRelativeString = useTimeAgo(lastSaved, { updateInterval: 800, showSecond: true });

const { theme } = useTheme(null);

const ratingLerpBind = (value: number) => ratingLerp(value, theme.value);

beginFetch();
</script>