<template>
  <nav class="flex items-center gap-6">
    <p v-if="name" class="text-lg font-thin text-colors-primary-200 lowercase">
      {{ name }}
    </p>
    <NuxtLink
        v-for="link in links"
        :key="link.to"
        :to="link.to"
        class="text-colors-primary-400 hover:text-colors-primary-200 transition-colors"
    >
      {{ link.text }}
    </NuxtLink>
  </nav>
</template>

<script lang="ts" setup>
defineProps<{ name?: string; }>();

const links = [
  { to: '/page', text: 'page' },
  { to: '/pages', text: 'pages' },
  { to: '/groups', text: 'groups' },
  { to: '/theme', text: 'theme' },
  { to: '/logout', text: 'logout' },
];
</script>