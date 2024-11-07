<template>
  <div class="flex flex-col flex-grow items-center justify-center relative px-4">
    <div class="relative space-y-12 max-w-2xl w-full text-center">
      <div class="space-y-6">
        <h1 class="text-6xl sm:text-8xl font-light tracking-tight text-colors-primary-100">
          jrnl.fm
        </h1>
        <p class="text-xl sm:text-2xl text-colors-primary-300 font-light">
          the simplest journal app and emotion tracker
        </p>
      </div>

      <div class="pt-8">
        <NuxtLink
            v-if="!!jwt"
            to="/page"
            class="inline-flex items-center justify-center px-8 py-4 text-lg rounded-full bg-colors-primary-600/80 hover:bg-colors-primary-500/80 text-colors-primary-100 transition-all duration-200 backdrop-blur-sm group"
        >
          open your journal
        </NuxtLink>
        <NuxtLink
            v-else
            to="/login"
            class="inline-flex items-center justify-center px-8 py-4 text-lg rounded-full bg-colors-primary-800/80 hover:bg-colors-primary-700/80 text-colors-primary-100 transition-all duration-200 backdrop-blur-sm group"
        >
          login w google
        </NuxtLink>
      </div>
    </div>

  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();

definePageMeta({ redirectUnautheticated: false });
</script>