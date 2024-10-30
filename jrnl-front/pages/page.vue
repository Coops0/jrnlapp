<template>
  <div>
    <p v-if="status === 'pending'">loading</p>
    <p v-else-if="status === 'error'">error</p>
    <div v-else-if="status === 'success'">
      <h3 v-if="showTimeUntil" @click="() => toggleTimeUntil()">time left {{ tomorrowRelativeString }}</h3>
      <h3 v-else @click="() => toggleTimeUntil()">show time left</h3>
      <h5>last saved {{ lastSavedRelativeString }}</h5>
      <textarea name="" id="" cols="30" rows="10" v-model="entry.text"/>
      <input type="number" v-model="entry.emotion_scale" min="0" max="10"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import { EntryService } from '~/services/entry.service';
import { getTomorrow, isSameDay } from '~/util/index.util';
import type { Entry } from '~/types/entry.type';

const { $localApi } = useNuxtApp();
const entryService = new EntryService($localApi);

const BLANK_ENTRY = () => ({ text: '', emotion_scale: 5, date: new Date().toString() } as Entry);

const todayStorage = useLocalStorage('entry-today', BLANK_ENTRY());
const entry = ref(todayStorage.value);

const { data, status } = useLazyAsyncData(
    'entry-today',
    async () => {
      const t = await entryService.getToday();
      todayStorage.value = { ...t };
      entry.value = { ...t };

      return t;
    },
    {
      default() {
        const t = todayStorage.value;

        if (isSameDay(new Date(t.date))) {
          return t;
        }

        todayStorage.value = BLANK_ENTRY();
        return BLANK_ENTRY();
      }
    }
);

const lastSaved = ref(new Date());
const lastSavedRelativeString = useTimeAgo(lastSaved);
const hasSavedBefore = ref(false);

const tomorrow = ref(getTomorrow());
const tomorrowRelativeString = useTimeAgo(tomorrow);

const showTimeUntil = useLocalStorage('show-time-until', true);
const toggleTimeUntil = useToggle(showTimeUntil);

watchDebounced(entry, save, { deep: true, debounce: 600, maxWait: 1000 });

useIntervalFn(async () => {
  const tmrw = getTomorrow();
  if (isSameDay(tmrw, tomorrow.value)) {
    return;
  }

  tomorrow.value = tmrw;

  entry.value = BLANK_ENTRY();
  todayStorage.value = BLANK_ENTRY();
}, 1000);


async function save() {
  if (!entry.value) {
    return;
  }

  // skip initial load
  if (!hasSavedBefore.value) {
    hasSavedBefore.value = true;
    return;
  }

  todayStorage.value = { ...entry.value };
  await entryService.putToday(entry.value.emotion_scale, entry.value.text);

  lastSaved.value = new Date();
}
</script>

