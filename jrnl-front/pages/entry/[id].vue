<template>
  <div class="w-full max-w-2xl mx-auto">
    <div v-if="status === 'pending'" class="flex justify-center items-center min-h-[50vh]">
      <p class="text-colors-primary-400">loading your entry...</p>
    </div>

    <div v-if="error" class="p-4 rounded-lg bg-colors-primary-900/40">
      <p class="text-colors-accent-400">unable to load entry: {{ error }}</p>
    </div>

    <div v-else-if="entry" class="space-y-8">
      <div class="relative">
        <div
            class="absolute inset-0 h-32 rounded-xl opacity-20 blur-xl transition-opacity"
            :style="{ 'background-color': ratingLerp(entry.emotion_scale, theme) }"
        />

        <div class="relative flex flex-col sm:flex-row items-center gap-4 p-6">
          <div class="flex-1">
            <p class="text-colors-primary-200 text-sm">
              {{
                parsedDate?.toLocaleDateString('en-US', {
                  weekday: 'long',
                  year: 'numeric',
                  month: 'long',
                  day: 'numeric'
                })
              }}
            </p>
          </div>

          <div class="flex items-center gap-3">
            <div
                class="w-5 h-5 rounded-full border-2 border-colors-primary-800/50"
                :style="{ 'background-color': ratingLerp(entry.emotion_scale, theme) }"
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
          class="prose prose-invert prose-p:text-colors-primary-200 prose-headings:text-colors-primary-100 max-w-none px-4"
          v-html="entry.text"
      />

      <div
          v-else
          class="px-4 py-12 text-center rounded-lg bg-colors-primary-900/20"
      >
        <p class="text-colors-primary-400 text-sm">no entry text for this day</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { parseServerDate, ratingLerp } from '~/util/index.util';

const route = useRoute();
const { id } = route.params;

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { theme } = useTheme(null);

const { data: entry, error, status } = useLazyAsyncData(
    `entry-${id}`,
    () => entryService.getEntry(id as string)
);

const parsedDate = computed(() => entry.value ? parseServerDate(entry.value.date) : null);
</script>