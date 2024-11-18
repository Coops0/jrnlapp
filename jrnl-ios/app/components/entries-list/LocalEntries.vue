<template>
  <div class="w-full max-w-2xl mx-auto">
    <div class="space-y-4 mt-4">
      <div v-if="entries" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
        <EntriesListPastEntry
            v-for="entry in datedEntries"
            :id="entry.id"
            :key="entry.id"
            :color="ratingLerp(entry.emotion_scale, theme)"
            :date="entry.date"
            :rating="entry.emotion_scale"
        />
      </div>
      <div v-else class="p-4 rounded-lg">
        <p class="text-colors-primary-400">you haven't logged anything yet</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { parseServerDate, ratingLerp } from '~/util/index.util';
import { useLocalEntries } from '~/composables/local-entries.composable';

const { theme } = useTheme(null);
const { entries } = useLocalEntries();

const datedEntries = computed(() =>
    entries.value.map(entry => ({
      ...entry,
      date: parseServerDate(entry.date)
    }))
);
</script>