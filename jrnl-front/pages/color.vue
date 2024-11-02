<template>
  <div>
    <h1 class="text-colors-text-400">what's your favorite color?</h1>
    <div class="flex flex-row flex-wrap">
      <div v-for="(color, name) in colors" :key="name" class="m-2" @click="setTheme(name)">
        <div class="w-32 h-32 rounded-lg" :style="{ backgroundColor: color }" :class="{ 'ring-colors-accent-400 ring-1 ring-offset-1': profile?.favorite_color === name }"/>
        <p>{{ name }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ProfileService } from '~/services/profile.service';
import { useProfile } from '~/composables/profile.composable';

const { $localApi } = useNuxtApp();
const profileService = new ProfileService($localApi);

const { profile, refresh } = useProfile(profileService);

const colors = {
  purple: '#3129d6',
  plant: '#757d4f'
} as const;

async function setTheme(name: string) {
  if (!profile.value) {
    console.debug('profile is null, refreshing');
    await refresh();
  }

  profile.value!.favorite_color = name;
  await profileService.updateMe({ favorite_color: name });
}
</script>