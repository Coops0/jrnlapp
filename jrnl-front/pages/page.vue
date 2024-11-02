<template>
  <div>
    <ClientOnly>
      <h5 v-if="lastSaved.getFullYear() !== 1900">last saved {{ lastSavedRelativeString }}</h5>
      <TimeUntilTomorrow :tomorrow/>
      <TextEditor v-model="entry.text" class="w-full h-full"/>
      <Slider v-model="entry.emotion_scale" :min="0" :max="10" :step="0.1"/>
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { EntryService } from '~/services/entry.service';
import { useTodayEntry } from '~/composables/today-entry.composable';
import TimeUntilTomorrow from '~/components/TimeUntilTomorrow.vue';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { beginFetch, tomorrow, entry, lastSaved } = useTodayEntry(entryService);
const lastSavedRelativeString = useTimeAgo(lastSaved, { updateInterval: 800, showSecond: true });

// don't await... we have localstorage cache for until then
beginFetch();
</script>

