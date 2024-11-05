<template>
  <div>
    <div v-if="status === 'pending'">
      loading...
    </div>
    <div v-if="error">
      <div>Error: {{ error }}</div>
    </div>
    <div v-else-if="entry">
      <div>{{ entry.date }}</div>
      <div v-html="entry.text ?? 'no text'"></div>
      <div>{{ entry.emotion_scale }}</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';

const route = useRoute();
const { id } = route.params;

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { data: entry, error, status } = useLazyAsyncData(`entry-${id}`, () => entryService.getEntry(id as string));
</script>
