<template>
  <nav class="flex items-center gap-6">
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
  { to: '/current', text: 'current' },
  { to: '/past', text: 'past' },
  { to: '/groups', text: 'groups' },
  { to: '/logout', text: 'logout' },
];
</script>