<template>
  <div>
    <h1>Pages</h1>
    <div>
      <ClientOnly>
        <PastEntry
            v-for="entry in entries"
            :key="entry.id"
            :id="entry.id"
            :rating="entry.emotion_scale"
            :date="entry.date"
        />

        <div v-if="paginator.has_more" @click="loadMore">load more</div>
      </ClientOnly>
    </div>
  </div>
</template>

<script setup lang="ts">

import { EntryService, type StrippedEntry } from '~/services/entry.service';
import PastEntry from '~/components/PastEntry.vue';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const reactiveStorage = useLocalStorage('entry-cache', [] as StrippedEntry[]);
const entries = ref<StrippedEntry[]>([]);

const nextCursor = ref<string | null>(null);

const { data: paginator, execute } = useLazyAsyncData(
    'entries',
    () => entryService.getEntriesPaginated(nextCursor.value || undefined, 50),
    {
      default() {
        return {
          items: reactiveStorage.value,
          next_cursor: null,
          has_more: false
        };
      },
      watch: [nextCursor],
    }
);

onMounted(execute);

watch(paginator, (p) => {
  if (!p?.items?.length) {
    return;
  }

  for (const entry of p.items) {
    if (!entries.value.find(e => e.id === entry.id)) {
      entries.value.push(entry);
    }
  }

  reactiveStorage.value = [...entries.value];
}, { immediate: true, deep: true });

function loadMore() {
  if (paginator.value.has_more && paginator.value.next_cursor) {
    nextCursor.value = paginator.value.next_cursor;
  }
}
</script>