<template>
  <div class="text-colors-text-200">one min im logging you out the jrnl</div>
</template>

<script lang="ts" setup>
import { AuthService } from '~/services/auth.service';
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
    user.value = null;
    await navigateTo('/');
  }
});
</script>