<template>
  <nav class="flex items-center gap-6">
    <template v-if="!authenticated">
      <NuxtLink
          to="/login"
          class="px-4 py-2 rounded-full bg-colors-primary-800 text-colors-primary-100 hover:bg-colors-primary-700 transition-colors"
      >
        login
      </NuxtLink>
    </template>
    <template v-else>
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
    </template>
  </nav>
</template>

<script lang="ts" setup>
defineProps<{
  authenticated: boolean;
  name?: string;
}>();

const links = [
  { to: '/page', text: 'page' },
  { to: '/pages', text: 'pages' },
  { to: '/groups', text: 'groups' },
  { to: '/color', text: 'theme' },
  { to: '/logout', text: 'logout' },
];
</script>