<template>
<div>one min im logging you out the jrnl</div>
</template>

<script setup lang="ts">
const supabase = useSupabaseClient();

onMounted(async () => {
  try {
    await supabase.auth.signOut();
  } finally {
    await navigateTo('/login');
  }
});
</script>