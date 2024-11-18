<template>
  <div class="w-full">
    <EntryLocalEntry v-if="forceLocalOverride || !jwt"/>
    <EntryRemoteEntry v-else @force-local="forceLocal"/>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();
const forceLocalOverride = ref(false);

const forceLocal = () => {
  forceLocalOverride.value = true;
}
</script>