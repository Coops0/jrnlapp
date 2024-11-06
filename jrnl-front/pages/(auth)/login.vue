<template>
  <div>
    <button @click="signInWithGoogle">
      google
    </button>
  </div>
</template>

<script lang="ts" setup>
const { public: { apiBase } } = useRuntimeConfig();

definePageMeta({ redirectUnautheticated: false });

async function signInWithGoogle() {
  await navigateTo(`${apiBase}/auth/google`, { external: true });
}
</script>