<template>
  <div class="flex flex-col relative flex-grow">
    <div class="top-2 left-0 right-0 px-4 z-10 mb-2">
      <div class="flex justify-between items-center max-w-5xl mx-auto text-sm">
        <ClientOnly>
          <h5
              class="text-colors-primary-400/80 hover:text-colors-primary-400 transition-opacity">
            {{ lastSaved.getFullYear() !== 1900 ? `last saved ${lastSavedRelativeString}` : '' }}
          </h5>
          <TimeUntilTomorrow :tomorrow="tomorrow"/>
        </ClientOnly>
      </div>
    </div>

    <ClientOnly>
      <div class="flex flex-col flex-grow">
        <TextEditor
            v-model="entry.text"
            class="flex-grow h-full w-full"
        />

        <div class="flex-grow mt-auto px-4 py-3 bg-colors-primary-900/50">
          <div class="space-y-2 max-w-5xl mx-auto">
            <Slider
                v-model="entry.emotion_scale"
                :max="10"
                :min="0"
                :step="0.1"
                :rating-lerp="ratingLerp"
            />
          </div>
        </div>
      </div>
    </ClientOnly>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { useTodayEntry } from '~/composables/today-entry.composable';
import TimeUntilTomorrow from '~/components/TimeUntilTomorrow.vue';
import Slider from '~/components/form/Slider.vue';
import { ratingLerp } from '~/util/index.util';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { beginFetch, tomorrow, entry, lastSaved } = useTodayEntry(entryService);
const lastSavedRelativeString = useTimeAgo(lastSaved, { updateInterval: 800, showSecond: true });

beginFetch();
</script>