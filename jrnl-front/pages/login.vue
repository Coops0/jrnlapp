<template>
  <div>
    <Button @click="signInWithGoogle">
      google
    </Button>
  </div>
</template>

<script setup lang="ts">
const supabase = useSupabaseClient();
const { base } = useRuntimeConfig().public;

async function signInWithGoogle() {
  const { error } = await supabase.auth.signInWithOAuth({
    provider: 'google', options: {
      redirectTo: `${base}/confirm`
    }
  });

  if (error) {
    console.error(error);
  }
}
</script>