<template>
  <div>
    <ClientOnly>
      <h5 v-if="lastSaved.getFullYear() !== 1900">last saved {{ lastSavedRelativeString }}</h5>
      <TimeUntilTomorrow :tomorrow/>
    </ClientOnly>
    <TextEditor v-model="entry.text" class="w-full h-full"/>
    <Slider v-model="entry.emotion_scale" :max="10" :min="0" :step="0.1"/>
  </div>
</template>

<script lang="ts" setup>
import { EntryService } from '~/services/entry.service';
import { useTodayEntry } from '~/composables/today-entry.composable';
import TimeUntilTomorrow from '~/components/TimeUntilTomorrow.vue';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { beginFetch, tomorrow, entry, lastSaved } = useTodayEntry(entryService);
const lastSavedRelativeString = useTimeAgo(lastSaved, { updateInterval: 800, showSecond: true });

beginFetch();
</script>

