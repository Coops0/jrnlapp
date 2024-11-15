<template>
  <div class="w-full mx-auto">
    <div v-if="entry" class="space-y-8">
      <EntryView
          :entry
          :parsed-date="parsedDate"
          :theme
      />
    </div>
    <div v-else>
      <p class="text-colors-accent-400">entry not found</p>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { parseServerDate } from '~/util/index.util';

const route = useRoute();
const { id } = route.params;

const { theme } = await useTheme(null);

const { entries } = await useLocalEntries();

const entry = computed(() => entries.value.find(entry => entry.id === id));

const parsedDate = computed(() => entry.value ? parseServerDate(entry.value.date) : null as never);
</script>