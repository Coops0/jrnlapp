<template>
  <div>

  </div>
</template>

<script setup lang="ts">

import { EntryService } from '~/services/entry.service';
import type { Entry } from '~/types/entry.type';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);
const storage = useLocalStorage('entry-cache', {} as Record<string, Entry>);

const { data: entries, status } = useAsyncData('entries', () => entryService.getEntriesPaginated(), {
  default() {
    return Object.values(storage.value).sort((a, b) => b.date - a.date);
  }
});

</script>