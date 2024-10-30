<template>
  <div>
    <h3 v-if="show" @click="() => toggle()">time left {{ timeUntilTomorrow }}</h3>
    <h3 v-else @click="() => toggle()">show time left</h3>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{ tomorrow: Date }>();

const timeUntilTomorrow = useTimeAgo(props.tomorrow);

const show = useLocalStorage('show-time-until', true);
const toggle = useToggle(show);
</script>

