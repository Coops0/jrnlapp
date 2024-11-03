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
  initialValue: profile.value?.theme ?? 'purple',
  attribute: 'data-theme',
  modes: {
    purple: 'purple',
    plant: 'plant'
  }
});

watchImmediate(profile, p => {
  if (p?.theme) {
    console.debug('layout: setting theme to', p.theme)
    theme.value = p.theme;
  }
}, { deep: true });
</script>