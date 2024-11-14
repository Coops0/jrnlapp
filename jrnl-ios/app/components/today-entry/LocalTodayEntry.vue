<template>
  <div class="flex flex-col h-full relative flex-grow px-2 md:px-4 lg:px-10 max-w-full">
    <div class="top-2 left-0 right-0 z-[2] mb-2">
      <div class="flex justify-between items-center mx-auto text-sm">
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
import { load } from '@tauri-apps/plugin-store';
import { useLocalTodayEntry } from '~/composables/local-today-entry.composable';
import { ratingLerp } from '~/util/index.util';

const { theme } = await useTheme(null);

const entryStore = await load('entry-today.json');

const { entry, tomorrow } = await useLocalTodayEntry(entryStore);

const ratingLerpBind = (rating: number) => ratingLerp(rating, theme.value);
</script>