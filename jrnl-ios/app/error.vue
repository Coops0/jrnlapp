<template>
  <div class="size-full bg-gray-800 text-gray-300">
    <div class="items-center p-5">
      <div>
        <h1 class="text-red-400">an error occurred</h1>
        <h2 class="text-red-300">{{ error.message }}</h2>
        <pre class="text-red-200">{{ error.stack }}</pre>
      </div>

      <div class="flex mt-4 flex-row gap-2 justify-center">
        <button class="px-4 py-2 bg-red-700 hover:bg-red-600 rounded-md text-white-100" @click="clear">clear error</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { NuxtError } from '#app';

defineProps<{ error: NuxtError }>();

const clear = () => clearError({ redirect: '/' });
</script>

<style>
body, html, #__nuxt {
  width: 100%;
  height: 100%
}
</style>