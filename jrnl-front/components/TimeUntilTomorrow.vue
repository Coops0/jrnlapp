<template>
  <div>
    <h3 class="text-color-text-300" v-if="show" @click="() => toggle()">time left {{ timeUntilTomorrow }}</h3>
    <h3 v-else @click="() => toggle()">show time left</h3>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{ tomorrow: Date }>();

const timeUntilTomorrow = useTimeAgo(props.tomorrow, { updateInterval: 800, showSecond: true });

const show = useLocalStorage('show-time-until', true);
const toggle = useToggle(show);
</script>

