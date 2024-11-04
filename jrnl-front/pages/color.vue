<template>
  <div>
    <h1 class="text-colors-text-400">what's your favorite color?</h1>
    <div class="flex flex-row flex-wrap">
      <div v-for="(color, name) in colors" :key="name" class="m-2" @click="setTheme(name)">
        <div class="w-32 h-32 rounded-lg" :style="{ backgroundColor: color }"
             :class="{ 'ring-colors-accent-400 ring-1 ring-offset-1': user?.theme === name }"/>
        <p>{{ name }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { UserService } from '~/services/user.service';
import { useUser } from '~/composables/user.composable';

const { $localApi } = useNuxtApp();
const userService = new UserService($localApi);

const { user } = useUser(userService);

const colors = {
  purple: '#3129d6',
  plant: '#757d4f'
} as const;

async function setTheme(name: string) {
  user.value!.theme = name;
  await userService.updateMe({ theme: name });
}
</script>