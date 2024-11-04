<template>
  <div>one min im logging you out the jrnl</div>
</template>

<script setup lang="ts">
import { AuthService } from '~/services/auth.service';
import { useUser } from '~/composables/user.composable';
import { UserService } from '~/services/user.service';

const { $localApi } = useNuxtApp();

const authService = new AuthService($localApi);
const userService = new UserService($localApi);

const { logout } = useAuth();
const { user } = useUser(userService);

onMounted(async () => {
  try {
    await authService.logout();
  } finally {
    logout();
    user.value = {};
    await navigateTo('/');
  }
});
</script>