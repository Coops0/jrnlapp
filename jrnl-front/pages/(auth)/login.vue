<template>
  <div>
    <Button @click="signInWithGoogle">
      google
    </Button>
  </div>
</template>

<script lang="ts" setup>
const { public: { apiBase } } = useRuntimeConfig();

definePageMeta({ redirectUnautheticated: false });

async function signInWithGoogle() {
  await navigateTo(`${apiBase}/auth/google`, { external: true });
}
</script>