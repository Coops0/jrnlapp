<template>
  <span
      class="text-2xl font-semibold text-colors-primary-100 hover:text-colors-primary-300 transition-colors cursor-pointer"
      @click="cycleTheme"
  >
    jrnl
  </span>
</template>

<script lang="ts" setup>
import { UserService } from '~/services/user.service';
import { themes } from 'assets/themes';

const { $localApi } = useNuxtApp();
const userService = new UserService($localApi);

const { theme, setTheme } = useTheme(userService);

const indexedThemes = computed(() => Object.keys(themes).filter(t => t !== 'lunar_placeholder'));

function cycleTheme() {
  const currentIndex = indexedThemes.value.indexOf(theme.value) ?? 2;
  let nextIndex = currentIndex + 1;
  if (nextIndex >= indexedThemes.value.length) {
    nextIndex = 0;
  }

  const nextTheme = indexedThemes.value[nextIndex];
  if (nextTheme) {
    setTheme(nextTheme);
  } else {
    console.warn('next theme is undefined');
  }
}
</script>