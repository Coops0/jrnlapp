<template>
  <div>
    <header class="bg-colors-background-400 text-colors-text-400 p-4">
      <h1>
        <NuxtLink :to="jwt?.length ? '/page' : '/'">jrnl</NuxtLink>
      </h1>

      <h1 v-if="!jwt?.length">
        <NuxtLink to="/login">login</NuxtLink>
      </h1>
    </header>

    <main>
      <slot/>
    </main>

    <footer class="bg-colors-background-400 text-colors-text-400 p-4">
      <p>hm?</p>
    </footer>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();
</script>