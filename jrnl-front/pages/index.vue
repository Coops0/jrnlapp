<template>
  <div>
    <h1>
      you can't cheat your thoughts
      <NuxtLink v-if="!!jwt" to="/page">go to your jrnl</NuxtLink>
    </h1>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();
</script>