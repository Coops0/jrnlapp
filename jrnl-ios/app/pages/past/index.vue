<template>
 <div>
   <EntriesListLocalEntries v-if="forceLocalOverride || !jwt" />
   <EntriesListRemoteEntries v-else @force-local="forceLocal" />
 </div>
</template>

<script lang="ts" setup>
const { jwt } = await useAuth();
const forceLocalOverride = ref(false);

const forceLocal = () => {
  forceLocalOverride.value = true;
}
</script>