<template>
  <div class="text-right">
    <h3
        v-if="show"
        class="text-colors-primary-300 hover:text-colors-primary-200 cursor-pointer text-sm"
        @click="toggle"
    >
      day ends in
      <NuxtTime :datetime="tomorrow" relative/>
    </h3>
    <h3
        v-else
        class="text-colors-primary-500 hover:text-colors-primary-300 cursor-pointer text-sm px-4"
        @click="toggle"
    >
      ☀️
    </h3>
  </div>
</template>

<script lang="ts" setup>
defineProps<{ tomorrow: Date }>();

const show = useCookie('show-time-until', {
  default: () => true,
  maxAge: 60 * 60 * 24 * 365
});

const toggle = () => {
  show.value = !show.value;
};
</script>