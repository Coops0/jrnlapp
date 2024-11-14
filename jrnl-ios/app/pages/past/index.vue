<template>
 <div>
   <EntriesListLocalEntries v-if="!jwt" />
   <EntriesListRemoteEntries v-else />
 </div>
</template>

<script lang="ts" setup>
const { jwt } = await useAuth();
</script>