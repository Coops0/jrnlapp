<template>
  <div>
    <Button @click="signInWithGoogle">
      google
    </Button>
  </div>
</template>

<script setup lang="ts">
import { definePageMeta } from '#imports';

const { public: { apiBase } } = useRuntimeConfig();

definePageMeta({
  redirectUnautheticated: false,
});

async function signInWithGoogle() {
  await navigateTo(`${apiBase}/auth/google`, { external: true });
}
</script>