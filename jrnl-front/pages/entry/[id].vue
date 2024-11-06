<template>
  <div>
    <div v-if="status === 'pending'">
      loading...
    </div>
    <div v-if="error">
      <div>Error: {{ error }}</div>
    </div>
    <div v-else-if="entry">
      <div class="flex flex-row justify-evenly">
        <p class="text-2xl text-colors-text-200">{{ entry.date }}</p>
        <div class="w-16 h-16 rounded-full" :style="{'background-color': ratingLerp(entry.emotion_scale)}"></div>
      </div>
      <p class="prose prose-invert max-w-none" v-html="entry.text ?? 'no text'"></p>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { ratingLerp } from '~/util/index.util';

const route = useRoute();
const { id } = route.params;

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { data: entry, error, status } = useLazyAsyncData(`entry-${id}`, () => entryService.getEntry(id as string));
</script>
