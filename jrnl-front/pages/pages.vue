<template>
  <div class="w-full max-w-2xl mx-auto">
    <div v-if="status === 'pending'">
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
        <EntriesListPastEntry
            id=""
            :color="ratingLerp(5, theme)"
            :date="new Date().toString()"
            :disabled="true"
            :rating="5"
        />
      </div>
    </div>

    <div v-else-if="error" class="p-4 rounded-lg bg-colors-primary-900/40">
      <p class="text-colors-accent-400">unable to load entries: {{ error }}</p>
    </div>

    <div v-else-if="paginator" class="space-y-4">
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
        <EntriesListPastEntry
            v-for="entry in paginator.items"
            :id="entry.id"
            :key="entry.id"
            :color="ratingLerp(entry.emotion_scale, theme)"
            :date="entry.date"
            :rating="entry.emotion_scale"
        />
      </div>

      <button
          v-if="paginator.has_more"
          class="w-full p-2 text-center text-sm text-colors-primary-400 bg-colors-primary-900/40 hover:bg-colors-primary-800/60 rounded-lg transition-colors duration-200"
          @click="loadMore"
      >
        load more
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService, type StrippedEntry } from '~/services/entry.service';
import { isSameDay, parseServerDate, ratingLerp } from '~/util/index.util';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { theme } = useTheme(null);

const nextCursor = ref<string | null>(null);
const { data: paginator, status, error } = useLazyAsyncData(
    'entries',
    () => entryService.getEntriesPaginated(nextCursor.value || undefined, 50),
    {
      watch: [nextCursor],
      transform(p) {
        const entries: StrippedEntry[] = [...(paginator.value?.items ?? [])];

        for (const entry of (p?.items ?? [])) {
          if (!entries.find((e: StrippedEntry) => e.id === entry.id)) {
            entries.push(entry);
          }
        }

        return {
          ...p,
          // don't show today entry
          items: entries.filter(e => !isSameDay(parseServerDate(e.date)))
        };
      }
    }
);

function loadMore() {
  if (paginator.value?.has_more && paginator.value?.next_cursor) {
    nextCursor.value = paginator.value.next_cursor;
  }
}
</script>