<template>
  <div class="text-right">
    <h3
        v-if="show"
        class="text-colors-primary-300 hover:text-colors-primary-200 cursor-pointer text-sm"
        @click="() => toggle()"
    >
      day ends in {{ timeUntilTomorrow }}
    </h3>
    <h3
        v-else
        class="text-colors-primary-500 hover:text-colors-primary-300 cursor-pointer text-sm"
        @click="() => toggle()"
    >
      show time left
    </h3>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{ tomorrow: Date }>();

const timeUntilTomorrow = useTimeAgo(props.tomorrow, { updateInterval: 800, showSecond: true });

const show = useLocalStorage('show-time-until', true);
const toggle = useToggle(show);
</script>