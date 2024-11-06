<template>
  <div class="w-full">
    <div v-if="paginator" class="flex flex-row gap-2 justify-evenly w-full">
      <EntriesListPastEntry
          v-for="entry in paginator.items"
          :id="entry.id"
          :key="entry.id"
          :date="entry.date"
          :rating="entry.emotion_scale"
      />

      <div v-if="paginator.has_more" @click="loadMore">load more</div>
    </div>
    <div v-else>
      <p class="text-colors-text-300 text-xl text-center">loading...</p>
    </div>
  </div>
</template>

<script lang="ts" setup>

import { EntryService, type StrippedEntry } from '~/services/entry.service';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const nextCursor = ref<string | null>(null);

const { data: paginator } = useLazyAsyncData(
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
          items: entries
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