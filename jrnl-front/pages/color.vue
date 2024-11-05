<template>
  <div>
    <h1 class="text-colors-text-400">what's your favorite color?</h1>
    <div class="flex flex-row flex-wrap">
      <div v-for="(color, name) in colors" :key="name" class="m-2" @click="setTheme(name)">
        <div :class="{ 'ring-colors-accent-400 ring-1 ring-offset-1': theme === name }" :style="{ backgroundColor: color }"
             class="w-32 h-32 rounded-lg"/>
        <p>{{ name }}</p>
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
  plant: '#757d4f'
} as const;
</script>