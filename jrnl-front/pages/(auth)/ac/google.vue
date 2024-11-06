<template>
  <div>
    hold up! {{ status }}
    <div v-if="error" class="text-red-500">{{ error }}</div>
  </div>
</template>

<script lang="ts" setup>
import { AuthService } from '~/services/auth.service';
import { UserService } from '~/services/user.service';

const route = useRoute();

const { $localApi } = useNuxtApp();
const authService = new AuthService($localApi);
const userService = new UserService($localApi);

const userComposable = useUser(userService);
const { jwt } = useAuth();

definePageMeta({ redirectUnautheticated: false });

const { data, error, status } = useLazyAsyncData(
    'google-oauth',
    () => authService.loginWithGoogle(route.query.state as string, route.query.code as string)
);

watch(data, async d => {
  if (!d) {
    return;
  }

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

