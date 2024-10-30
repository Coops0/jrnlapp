<template>
  <div>
    <h5 v-if="lastSaved.getFullYear() !== 1900">last saved {{ lastSavedRelativeString }}</h5>
    <TimeUntilTomorrow :tomorrow/>
    <textarea name="" id="" cols="30" rows="10" v-model="entry.text"/>
    <input type="range" v-model="entry.emotion_scale" min="0" max="10"/>
  </div>
</template>

<script setup lang="ts">
import { EntryService } from '~/services/entry.service';
import { useTodayEntry } from '~/composables/today-entry.composable';
import TimeUntilTomorrow from '~/components/TimeUntilTomorrow.vue';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const { beginFetch, tomorrow, entry, lastSaved } = useTodayEntry(entryService);
const lastSavedRelativeString = useTimeAgo(lastSaved);

// don't await... we have localstorage cache for until then
beginFetch();
</script>

