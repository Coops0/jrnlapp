<template>
  <div>
    hold up!
    <div v-if="error" class="text-red-500">{{ error }}</div>
  </div>
</template>

<script setup lang="ts">
import { AuthService } from '~/services/auth.service';
import { useUser } from '~/composables/user.composable';
import { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';
import { definePageMeta } from '#imports';

const route = useRoute();

const { $localApi } = useNuxtApp();
const authService = new AuthService($localApi);
const userService = new UserService($localApi);

const userComposable = useUser(userService);

definePageMeta({
  redirectUnautheticated: false,
});

const error = ref<string | null>(null);

onMounted(async () => {
  let user: User;
  try {
    user = await authService.loginWithGoogle(route.query.state as string, route.query.code as string);
  } catch (e) {
    console.error(e);
    error.value = e?.toString() ?? 'error logging in';
    return;
  }

  userComposable.user.value = user;

  try {
    await userService.updateMe({ tz: Intl.DateTimeFormat().resolvedOptions().timeZone });
  } catch (e) {
    console.warn(e);
  }

  await navigateTo('/page');
});
</script>

