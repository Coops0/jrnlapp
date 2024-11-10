<template>
  <NuxtLink
      class="text-2xl font-semibold text-colors-primary-100 hover:text-colors-primary-300 transition-all ease-in duration-150 bg-transparent"
      :class="{ 'glow-logo': isOnCurrentPage }"
      to="/current"
      @mousedown="startHold"
      @mouseup="endHold"
      @touchstart.passive="startHold"
      @touchend.passive="endHold"
      @touchmove.passive="endHold"
  >
    jrnl
  </NuxtLink>
</template>

<script lang="ts" setup>
const route = useRoute();

const isOnCurrentPage = computed(() => route.name === 'current');

const holdStartTime = ref<null | number>(null);

const startHold = () => {
  holdStartTime.value = Date.now();
};

const endHold = async () => {
  if (holdStartTime.value === null) {
    return;
  }

  const holdTime = Date.now() - holdStartTime.value;
  holdStartTime.value = null;

  if (holdTime > 3500) {
    await navigateTo('/logout');
  }
};
</script>

<style scoped>
.glow-logo {
  /*noinspection CssUnresolvedCustomProperty*/
  text-shadow: hsl(var(--twc-colors-primary-500)) 0 0 16px;
}
</style>