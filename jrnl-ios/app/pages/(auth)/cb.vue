<template>
  <div class="size-full flex-grow flex items-center justify-center p-6">
    <div class="w-full max-w-md text-center">
      <div v-if="!error" class="space-y-4">
        <div
            class="animate-spin w-12 h-12 border-4 border-colors-accent-400 border-t-transparent rounded-full mx-auto"/>
        <p class="text-colors-text-200 text-lg">signing you in...</p>
      </div>

      <div v-else class="bg-colors-primary-800/50 rounded-xl p-8 backdrop-blur-sm border border-colors-primary-700">
        <div class="text-red-400 mb-4">
          <span class="text-lg">login failed</span>
        </div>
        <p class="text-colors-text-300 mb-6">{{ error }}</p>
        <NuxtLink
            to="/login"
            class="inline-block px-6 py-2 rounded-lg bg-colors-primary-700 text-colors-text-200 hover:bg-colors-primary-600 transition-colors"
        >
          try again
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { UserService } from '~/services/user.service';
import type { User } from '~/types/user.type';
import { EntryService } from '~/services/entry.service';

definePageMeta({ redirectUnautheticated: false });

const route = useRoute();
const { $localApi } = useNuxtApp();

const userService = new UserService($localApi);

const { user, hasRefreshedRemotely } = await useUser(userService);
const { jwt } = await useAuth();

type QueryPayload = { error: string } | { user: User, token: string };

const error = ref<string | null>(null);
onMounted(async () => {
  let data: QueryPayload;
  try {
    data = JSON.parse(atob(route.query['r'] as string));
  } catch (e) {
    console.error(e);
    error.value = 'bad r callback query param';
    return;
  }

  if ('error' in data) {
    console.error(data.error);
    error.value = data.error;
    return;
  }

  jwt.value = data.token;

  hasRefreshedRemotely.value = true;
  user.value = data.user;

  try {
    await userService.updateMe({ tz: Intl.DateTimeFormat().resolvedOptions().timeZone });
  } catch (e) {
    console.warn(e);
  }


  try {
    const { entries } = await useLocalEntries();
    if (entries.value?.length) {
      await new EntryService($localApi).putPastEntries(entries.value);
    }
  } catch (e) {
    console.warn(e);
  }

  await navigateTo('/current');
});
</script>