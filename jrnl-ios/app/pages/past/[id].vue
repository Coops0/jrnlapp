<template>
  <div>
    <EntryLocalEntry v-if="!jwt"/>
    <EntryRemoteEntry v-else/>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = await useAuth();
</script>