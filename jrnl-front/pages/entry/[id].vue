<template>
  <div class="w-full max-w-2xl mx-auto">
    <div v-if="status === 'pending'" class="space-y-8">
      <EntryView
          :entry="{
            emotion_scale: 5,
            author: '',
            date: new Date().toString(),
            text: 'loading...',
            id: '',
          }"
          :parsed-date="new Date()"
          :theme
      />
    </div>

    <div v-else-if="error" class="p-4 rounded-lg bg-colors-primary-900/40">
      <p class="text-colors-accent-400">unable to load entry: {{ error }}</p>
    </div>

    <div v-else-if="entry" class="space-y-8">
      <EntryView
          :entry
          :parsed-date="parsedDate"
          :theme
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { parseServerDate } from '~/util/index.util';

const route = useRoute();
const { id } = route.params;

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { theme } = useTheme(null);

const { data: entry, error, status } = useLazyAsyncData(
    `entry-${id}`,
    () => entryService.getEntry(id as string)
);

const parsedDate = computed(() => entry.value ? parseServerDate(entry.value.date) : null as never);
</script>