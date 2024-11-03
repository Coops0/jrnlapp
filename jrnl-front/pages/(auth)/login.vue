<template>
  <div>
    <Button @click="signInWithGoogle">
      google
    </Button>
  </div>
</template>

<script setup lang="ts">
import { AuthService } from '~/services/auth.service';
import { definePageMeta } from '#imports';

const { $localApi } = useNuxtApp();
const authService = new AuthService($localApi);

definePageMeta({
  redirectUnautheticated: false,
});

async function signInWithGoogle() {
  const redirectResponse = await authService.getGoogleRedirect() as unknown as Response;
  await navigateTo(redirectResponse.url);
}
</script>