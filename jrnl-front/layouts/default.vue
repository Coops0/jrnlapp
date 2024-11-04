<template>
  <div>
    <header class="bg-colors-background-400 text-colors-text-400 p-4">
      <h1>
        <NuxtLink to="/">jrnl</NuxtLink>
      </h1>
    </header>

    <main>
      <slot/>
    </main>

    <footer class="bg-colors-background-400 text-colors-text-400 p-4">
      <p>hm?</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { UserService } from '~/services/user.service';
import { useUser } from '~/composables/user.composable';

const { $localApi } = useNuxtApp();
const userService = new UserService($localApi);
const { user } = useUser(userService);

const theme = useColorMode({
  initialValue: user.value?.theme ?? 'purple',
  attribute: 'data-theme',
  modes: {
    purple: 'purple',
    plant: 'plant'
  }
});

watchImmediate(user, p => {
  if (p?.theme) {
    console.debug('layout: setting theme to', p.theme);
    theme.value = p.theme;
  }
}, { deep: true });
</script>