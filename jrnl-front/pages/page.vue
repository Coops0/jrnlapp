<template>
  <div>
    <p v-if="status === 'pending'">loading</p>
    <p v-else-if="status === 'error'">error</p>
    <div v-else-if="status === 'success'">
      <h3>time left {{ formatTimeAgo(tomorrow) }}</h3>
      <h5>last saved {{ formatTimeAgo(lastSaved) }}</h5>
      <textarea name="" id="" cols="30" rows="10" v-model="text"/>
      <input type="number" v-model="rating" min="0" max="10"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import { EntryService } from '~/services/entry.service';
import { formatTimeAgo } from '@vueuse/core';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);
const storage = useLocalStorage('entry-today', { text: '', rating: 5, date: Date.now() });

const { data: entry, status } = useAsyncData('entry-today', () => entryService.getToday());

const text = ref('');
const rating = ref(5);

const lastSaved = ref(new Date());
const tomorrow = ref(getTomorrow());

watchOnce(storage, s => {
  if (isSameDay(new Date(s.date), new Date())) {
    text.value = s.text;
    rating.value = s.rating;
  } else {
    storage.value = { text: '', rating: 5, date: Date.now() };
  }
});

watchOnce(entry, (e) => {
  if (e) {
    text.value = e.text || text.value;
    rating.value = e.emotion_scale;
  }
});

watchDebounced([text, rating], save, { debounce: 600, maxWait: 1000 });

useIntervalFn(async () => {
  const tmrw = getTomorrow();
  if (tmrw.getTime() === tomorrow.value.getTime()) {
    return;
  }

  tomorrow.value = tmrw;
  await save();

  storage.value = { text: '', rating: 5, date: Date.now() };
}, 1000);

function getTomorrow(): Date {
  const tmrw = new Date();
  tmrw.setHours(0, 0, 0);
  tmrw.setDate(tmrw.getDate() + 1);

  return tmrw;
}

async function save() {
  storage.value = { text: text.value, rating: rating.value, date: Date.now() };
  await entryService.putToday(rating.value, text.value);

  lastSaved.value = new Date();
}

const isSameDay = (a: Date, b: Date) =>
    a.getDate() === b.getDate() &&
    a.getMonth() === b.getMonth() &&
    a.getFullYear() === b.getFullYear();

</script>

