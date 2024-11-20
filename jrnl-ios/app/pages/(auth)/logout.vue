<template>
 <div />
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
  localStorage.removeItem('jwt');
  localStorage.removeItem('user');
  localStorage.removeItem('theme');

  await navigateTo('/');
});
</script>