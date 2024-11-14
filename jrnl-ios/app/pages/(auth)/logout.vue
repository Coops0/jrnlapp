<template>
  <div class="size-full flex-grow flex items-center justify-center p-6">
    <div class="w-full max-w-md text-center">
      <div class="bg-colors-primary-800/50 rounded-xl p-8 backdrop-blur-sm border border-colors-primary-700">
        <div class="text-red-400 mb-4">
          <span class="text-lg">logging you in</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { UserService } from '~/services/user.service';

const { $localApi } = useNuxtApp();

const userService = new UserService($localApi);

const { logout } = await useAuth();
const { user } = await useUser(userService);

onMounted(async () => {
  await logout();
  user.value = null;
  useCookie('theme-cache').value = null;
  useCookie('entry-today').value = null;

  await navigateTo('/');
});
</script>