<template>
  <div class="w-full mx-auto">
    <div v-if="entry" class="space-y-8">
      <EntryView
          :entry
          :parsed-date="parsedDate"
          :theme
      />
    </div>
    <div v-else-if="!hasTriedFetch" class="space-y-8">
      <EntryView
          :entry="{
            emotion_scale: 5,
            author: '',
            date: new Date().toString(),
            text: 'loading...',
            id: '',
            saved: false
          }"
          :parsed-date="new Date()"
          :theme
      />
    </div>
    <div v-else class="p-4 rounded-lg justify-center">
      <p class="text-colors-accent-400">unable to load entry</p>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { parseServerDate } from '~/util/index.util';
import type { Entry } from '~/types/entry.type';
import { LocalBackendService } from '~/services/local-backend.service';
import { useOnline } from '~/composables/util/online.util.composable';

const route = useRoute();
const { id } = route.params;

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);
const localBackendService = new LocalBackendService();

const { jwt } = useAuth();
const { theme } = useTheme(null);
const {isOnline} = useOnline();

const entry = ref<Entry | null>(null);

const hasTriedFetch = ref(false);

async function fetchEntry() {
  entry.value = await localBackendService.getEntry(id as string);

  if (!jwt.value || !isOnline.value) {
    return;
  }

  try {
    entry.value = await entryService.getEntry(id as string);
    await localBackendService.saveEntry({ ...entry.value!, saved: true });
  } catch {
    /* empty */
  } finally {
    hasTriedFetch.value = true;
  }
}

fetchEntry();

const parsedDate = computed(() => entry.value ? parseServerDate(entry.value.date) : null as never);
</script>