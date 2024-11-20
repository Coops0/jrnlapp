<template>
  <div/>
</template>

<script lang="ts" setup>
import { UserService } from '~/services/user.service';

const { $localApi } = useNuxtApp();

const userService = new UserService($localApi);

const { logout } = useAuth();
const { user } = useUser(userService);

onMounted(async () => {
  logout();
  user.value = null;
  useCookie('theme-cache').value = null;
  useCookie('entry-today').value = null;

  await navigateTo('/');
});
</script>