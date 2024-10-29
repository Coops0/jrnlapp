<template>
  <div>
    <Button @click="signInWithGoogle">
      google
    </Button>
  </div>
</template>

<script setup lang="ts">
const supabase = useSupabaseClient();

async function signInWithGoogle() {
  const { error } = await supabase.auth.signInWithOAuth({ provider: 'google' });

  if (error) {
    console.error(error);
  }
}
</script>