<template>
  <div>
    <ClientOnly>
      hold up! {{ status }}
      <div v-if="error" class="text-red-500">{{ error }}</div>
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { AuthService } from '~/services/auth.service';
import { useUser } from '~/composables/user.composable';
import { UserService } from '~/services/user.service';

const route = useRoute();

const { $localApi } = useNuxtApp();
const authService = new AuthService($localApi);
const userService = new UserService($localApi);

const userComposable = useUser(userService);
const { jwt } = useAuth();

definePageMeta({
  redirectUnautheticated: false,
});

const { data, error, status } = useAsyncData(
    'google-oauth',
    () => authService.loginWithGoogle(route.query.state as string, route.query.code as string),
    {
      server: false
    }
);

watch(data, async d => {
  if (!d) return;

  jwt.value = d.token;
  userComposable.user.value = d.user;

  try {
    await userService.updateMe({ tz: Intl.DateTimeFormat().resolvedOptions().timeZone });
  } catch (e) {
    console.warn(e);
  }

  await navigateTo('/page');
}, { immediate: true, deep: true });

</script>

