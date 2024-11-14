<template>
  <div>
    <div class="relative">
      <div
          :style="{ 'background-color': ratingLerp(entry.emotion_scale, theme) }"
          class="absolute inset-0 h-32 rounded-xl opacity-20 blur-xl"
      />

      <div class="relative flex flex-col sm:flex-row items-center gap-4 p-6">
        <div class="flex-1">
          <NuxtTime
              v-if="parsedDate"
              :datetime="parsedDate" class="text-colors-primary-200 text-sm" day="numeric" month="long" weekday="long"
              year="numeric"
          />
        </div>

        <div class="flex items-center gap-3">
          <div
              :style="{ 'background-color': ratingLerp(entry.emotion_scale, theme) }"
              class="w-5 h-5 rounded-full border-2 border-colors-primary-800/50"
          />
          <span class="text-colors-primary-200 text-sm font-light">
              {{ entry.emotion_scale.toFixed(1) }}
            </span>
        </div>
      </div>
    </div>

    <!-- eslint-disable vue/no-v-html -->
    <div
        v-if="entry.text"
        class="prose prose-p:text-colors-primary-200 prose-headings:text-colors-primary-100 max-w-full px-4"
        v-html="entry.text"
    />

    <div
        v-else
        class="px-4 py-12 text-center rounded-lg"
    >
      <p class="text-colors-primary-400 text-sm">no entry text for this day</p>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { parseServerDate, ratingLerp } from '~/util/index.util';
import type { Entry } from '~/types/entry.type';

const props = defineProps<{
  entry: Entry;
  theme: string;
}>();

const parsedDate = computed(() => parseServerDate(props.entry.date));
</script>

