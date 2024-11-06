<template>
  <div class="flex flex-grow flex-row flex-wrap gap-2 justify-evenly w-full">
    <div v-for="(color, name) in colors" :key="name" class="m-2" @click="setTheme(name)">
      <div
          :class="{ 'ring-colors-text-500 ring-1 ring-offset-1': theme === name }"
          :style="{ backgroundColor: color }"
          class="w-32 h-32 rounded-full cursor-pointer">
        <p class="invert flex items-center justify-center h-full" :style="{ color }">{{ name }}</p>

      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { UserService } from '~/services/user.service';
import { useTheme } from '~/composables/theme.composable';

const { $localApi } = useNuxtApp();
const userService = new UserService($localApi);

const { theme, setTheme } = useTheme(userService);
const colors = {
  purple: '#3129d6',
  plant: '#757d4f',
  mocktail: '#290a5c',
  lunar: '#324a67',
  stone: 'black',
} as const;
</script>