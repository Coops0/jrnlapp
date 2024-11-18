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

    <div v-else-if="error" class="p-4 rounded-lg justify-center">
      <p class="text-colors-accent-400">unable to load entries ~ {{ error }}</p>
      <FormButton variant="secondary" size="md" @click="refresh">try again</FormButton>
    </div>

    <div v-else-if="paginator" class="space-y-4 mt-4">
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
import { EntryService } from '~/services/entry.service';
import { ratingLerp } from '~/util/index.util';
import { useRemoteEntries } from '~/composables/remote-entries.composable';

const emit = defineEmits<{
  forceLocal: []
}>();

const { theme } = useTheme(null);

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { loadMore, refresh, status, error, paginator } = useRemoteEntries(entryService);

watch(error, e => {
  if (e) {
    emit('forceLocal');
  }
});
</script>