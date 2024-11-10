<template>
  <div class="text-colors-text-200 text-center">
    <span>one moment please</span>
    <div v-if="error" class="text-red-500">{{ error }}</div>
  </div>
</template>

<script setup lang="ts">
import { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';

definePageMeta({ redirectUnautheticated: false });

const route = useRoute();
const { $localApi } = useNuxtApp();

const userService = new UserService($localApi);

const { user } = useUser(userService);
const { jwt } = useAuth();

type QueryPayload = { error: string } | { user: User, token: string };

const error = ref<string | null>(null);
onMounted(async () => {
  const data: QueryPayload = JSON.parse(atob(route.query['r'] as string));
  if ('error' in data) {
    console.error(data.error);
    error.value = data.error;
    return;
  }

  jwt.value = data.token;
  user.value = data.user;

  try {
    await userService.updateMe({ tz: Intl.DateTimeFormat().resolvedOptions().timeZone });
  } catch (e) {
    console.warn(e);
  }

  await navigateTo('/current');
});
</script>
