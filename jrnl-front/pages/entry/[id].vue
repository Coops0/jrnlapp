<template>
  <div>
    <div v-if="error">
      <div>Error: {{ error }}</div>
    </div>
    <div v-else-if="entry">
      <div>{{ entry.date }}</div>
      <div>{{ entry.text }}</div>
      <div>{{ entry.emotion_scale }}</div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { EntryService } from '~/services/entry.service';
import type { Entry } from '~/types/entry.type';

const route = useRoute();
const { id } = route.params;

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const localEntry = useLocalStorage(`entry-${id}`, null as Entry | null);


const { data: entry, error } = useLazyAsyncData(
    `entry-${id}`,
    () => entryService.getEntry(id as string),
    {
      default() {
        return localEntry;
      }
    }
);

watchImmediate(entry, () => {
  if (entry.value) {
    localEntry.value = entry.value;
  }
});
</script>

