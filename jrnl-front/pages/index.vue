<template>
  <div>
    <p>
      you can't cheat your thoughts
    </p>

    <NuxtLink v-if="!!jwt" to="/page">go to your jrnl</NuxtLink>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();
</script>