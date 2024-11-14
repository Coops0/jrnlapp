<template>
  <div class="size-full">
    <TodayEntryLocalTodayEntry v-if="!jwt"/>
    <TodayEntryRemoteTodayEntry v-else/>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = await useAuth();
</script>