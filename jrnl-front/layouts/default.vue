<template>
  <div>
    <header>
      <h1>
        <NuxtLink to="/">jrnl</NuxtLink>
      </h1>
    </header>

    <main>
      <slot/>
    </main>
    <footer>
      <p>hm?</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ProfileService } from '~/services/profile.service';

const { $localApi } = useNuxtApp();
const profileService = new ProfileService($localApi);
const { profile } = useProfile(profileService);

const theme = useColorMode({
  initialValue: profile.value?.favorite_color ?? 'purple',
  attribute: 'data-theme',
  modes: {
    purple: 'purple',
    plant: 'plant'
  }
});

watchImmediate(profile, p => {
  if (p?.favorite_color) {
    console.debug('layout: setting theme to', p.favorite_color)
    theme.value = p.favorite_color;
  }
}, { deep: true });
</script>