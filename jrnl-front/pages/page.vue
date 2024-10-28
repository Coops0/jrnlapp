<template>
  <div>
    <p v-if="status === 'pending'">loading</p>
    <p v-else-if="status === 'error'">error</p>
    <p v-else-if="status === 'success'">{{ JSON.stringify(entry) }}</p>
  </div>
</template>

<script setup lang="ts">
const { $localApi } = useNuxtApp();

const { data: entry, status } = useAsyncData('entry-today', () => $localApi('/entry/today'));


</script>

