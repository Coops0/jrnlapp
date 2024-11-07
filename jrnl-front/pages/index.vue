<template>
  <div class="flex flex-col flex-grow items-center justify-center relative px-4">
    <div class="relative space-y-12 max-w-2xl w-full text-center">
      <div class="space-y-6">
        <h1 class="text-6xl sm:text-8xl font-light tracking-tight text-colors-primary-100">
          jrnl
        </h1>
        <p class="text-xl sm:text-2xl text-colors-primary-300 font-light">
          you can't cheat your thoughts
        </p>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-4 gap-6 text-center">
        <div class="p-4 rounded-xl bg-colors-primary-900/30 backdrop-blur-sm">
          <p class="text-colors-primary-200 text-lg mb-2">simple</p>
          <p class="text-colors-primary-400 text-sm">literally the most minimalistic app possible</p>
        </div>

        <div class="p-4 rounded-xl bg-colors-primary-900/30 backdrop-blur-sm">
          <p class="text-colors-primary-200 text-lg mb-2">bereal but for a journal</p>
          <p class="text-colors-primary-400 text-sm">when the day ends, you can't edit your journal</p>
        </div>

        <div class="p-4 rounded-xl bg-colors-primary-900/30 backdrop-blur-sm">
          <p class="text-colors-primary-200 text-lg mb-2">easiest mood tracking</p>
          <p class="text-colors-primary-400 text-sm">literally just a slider... use it or don't</p>
        </div>

        <div class="p-4 rounded-xl bg-colors-primary-900/30 backdrop-blur-sm">
          <p class="text-colors-primary-200 text-lg mb-2">share together</p>
          <p class="text-colors-primary-400 text-sm">connect with friends in private groups optionally</p>
        </div>
      </div>

      <div class="pt-8">
        <NuxtLink
            v-if="!!jwt"
            to="/page"
            class="inline-flex items-center justify-center px-8 py-4 text-lg rounded-full bg-colors-primary-800/80 hover:bg-colors-primary-700/80 text-colors-primary-100 transition-all duration-200 backdrop-blur-sm group"
        >
          open your journal
          <span class="ml-2 group-hover:translate-x-1 transition-transform">→</span>
        </NuxtLink>
        <NuxtLink
            v-else
            to="/login"
            class="inline-flex items-center justify-center px-8 py-4 text-lg rounded-full bg-colors-primary-800/80 hover:bg-colors-primary-700/80 text-colors-primary-100 transition-all duration-200 backdrop-blur-sm group"
        >
          login w google
          <span class="ml-2 group-hover:translate-x-1 transition-transform">→</span>
        </NuxtLink>
      </div>
    </div>

  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();

definePageMeta({ redirectUnautheticated: false });
</script>