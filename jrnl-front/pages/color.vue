<template>
  <div class="flex flex-grow flex-row flex-wrap gap-2 justify-evenly w-full">
    <div v-for="(colors, name) in themes" :key="name" class="m-2" @click="setTheme(name)">
      <div
          :class="{ 'ring-colors-text-500 ring-1 ring-offset-1': theme === name }"
          :style="{ backgroundColor: colors.colors.primary[400] }"
          class="w-32 h-32 rounded-full cursor-pointer">
        <p class="flex items-center justify-center h-full" :style="{ color: colors.colors.text[50] }">{{ name }}</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { UserService } from '~/services/user.service';
import { themes } from '~/assets/themes';

const { $localApi } = useNuxtApp();
const userService = new UserService($localApi);

const { theme, setTheme } = useTheme(userService);
</script>