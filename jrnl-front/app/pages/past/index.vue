<template>
  <div class="w-full max-w-2xl mx-auto">
    <div v-if="status === 'pending'">
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
        <EntriesListPastEntry
            id=""
            :color="ratingLerp(5, theme)"
            :date="new Date()"
            disabled
            :rating="5"
        />
      </div>
    </div>

    <ErrorDisplay v-if="error" :error="error" @retry="refresh" />

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
      <div v-else class="p-4 rounded-lg">
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
import { watchErrorAndThrow } from '~/util/watch-error-and-throw.util';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { theme } = useTheme(null);

const nextCursor = ref<string | null>(null);
const limit = ref(50);

const additionalLoads = ref(0);

type StrippedEntryWithDate = Omit<StrippedEntry, 'date'> & { date: Date };

const { data: paginator, status, error, refresh } = useLazyAsyncData(
    'entries',
    () => entryService.getEntriesPaginated(nextCursor.value || undefined, limit.value),
    {
      watch: [nextCursor],
      transform(p) {
        const entries: StrippedEntryWithDate[] = [...(paginator.value?.items ?? [])];

        for (const entry of (p?.items ?? [])) {
          if (!entries.find((e: StrippedEntryWithDate) => e.id === entry.id)) {
            entries.push({
              ...entry,
              date: parseServerDate(entry.date)
            } as StrippedEntryWithDate);
          }
        }

        return {
          ...p,
          // don't show today entry
          items: entries.filter(e => !isSameDay(e.date))
        };
      }
    }
);

function loadMore() {
  if (paginator.value?.has_more && paginator.value?.next_cursor) {
    if (++additionalLoads.value > 3) {
      limit.value = 100;
    }

    nextCursor.value = paginator.value.next_cursor;
  }
}

watchErrorAndThrow(error);
</script>