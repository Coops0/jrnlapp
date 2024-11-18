<template>
  <div class="w-full max-w-2xl mx-auto">
    <div class="space-y-4 mt-4">
      <div v-if="entries.length" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
        <EntriesListPastEntry
            v-for="entry in entries"
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
          v-if="jwt && hasMore"
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
import { EntryService } from '~/services/entry.service';
import { ratingLerp } from '~/util/index.util';
import { LocalBackendService } from '~/services/local-backend.service';

const { jwt } = useAuth();
const { theme } = useTheme(null);

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);
const localBackendService = new LocalBackendService();

const {
  loadMore,
  loadLocalEntries,
  entries,
  hasMore
} = useEntries(localBackendService, jwt.value ? entryService : null);
loadLocalEntries();
</script>