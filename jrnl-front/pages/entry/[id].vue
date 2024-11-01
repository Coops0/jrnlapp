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

const localEntry = useLocalStorage(`entry-${id}`, {} as Entry);

const { data: entry, error, execute } = useLazyAsyncData(
    `entry-${id}`,
    () => entryService.getEntry(id as string),
    {
      default() {
        return localEntry;
      },
      transform(e) {
        localEntry.value = e;
        return e;
      },
    }
);

onMounted(execute);
</script>
