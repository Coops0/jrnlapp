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
      <FormButton variant="secondary" size="md" @click="refresh">try again</FormButton>
    </div>

    <div v-else-if="paginator" class="space-y-4">
      <div v-if="paginator.items.length" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
        <EntriesListPastEntry
            v-for="entry in paginator.items"
            :id="entry.id"
            :key="entry.id"
            :color="ratingLerp(entry.emotion_scale, theme)"
            :date="entry.date"
            :rating="entry.emotion_scale"
        />
      </div>
      <div v-else class="p-4 rounded-lg bg-colors-primary-900/40">
        <p class="text-colors-primary-400">you haven't logged anything yet</p>
      </div>

      <FormButton
          v-if="paginator.has_more"
          full
          size="sm"
          variant="secondary"
          @click="loadMore"
      >
        load more
      </FormButton>
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
const limit = ref(50);

const additonalLoads = ref(0);

const { data: paginator, status, error, refresh } = useLazyAsyncData(
    'entries',
    () => entryService.getEntriesPaginated(nextCursor.value || undefined, limit.value),
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
    if (++additonalLoads.value > 3) {
      limit.value = 100;
    }

    nextCursor.value = paginator.value.next_cursor;
  }
}
</script>