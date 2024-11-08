<template>
  <div class="flex flex-col relative flex-grow px-2 md:px-4 lg:px-10">
    <div class="top-2 left-0 right-0 z-10 mb-2">
      <div class="flex justify-between items-center max-w-5xl mx-auto text-sm">
        <h5
            class="text-colors-primary-400/80 hover:text-colors-primary-400 transition-opacity"
        >
          <span v-if="lastSaved.getFullYear() === 1900">last saved: never</span>
          <span v-else>last saved: <NuxtTime :datetime="lastSaved" relative/></span>
        </h5>
        <TodayEntryTimeUntilTomorrow :tomorrow="tomorrow"/>
      </div>
    </div>
    <div class="flex flex-col flex-grow">
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
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { ratingLerp } from '~/util/index.util';
import type { Entry } from '~/types/entry.type';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const entryCookie = useCookie<Entry>('entry-today');

const { beginFetch, tomorrow, entry, lastSaved } = useTodayEntry(entryService, entryCookie);

const { theme } = useTheme(null);

const ratingLerpBind = (value: number) => ratingLerp(value, theme.value);

beginFetch();
</script>