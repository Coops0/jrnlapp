<template>
  <div class="size-full bg-gray-800 text-gray-300">
    <div class="items-center">
      <div>
        <h1 class="text-red-400">an error occurred</h1>
        <h2 class="text-red-300">{{ error.message }}</h2>
        <pre class="text-red-200">{{ error.stack }}</pre>
      </div>

      <div>
        <button class="px-4 py-2 bg-red-700 hover:bg-red-600 text-white-100" @click="clear">clear error</button>
        <button class="px-4 py-2 bg-red-700 hover:bg-red-600 text-white-100" @click="clearCookiesAndRefresh">log out and
          refresh
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { NuxtError } from '#app';

defineProps<{ error: NuxtError }>();

function clearCookiesAndRefresh() {
  useCookie('theme-cache').value = null;
  useCookie('jwt').value = null;
  useCookie('cached-user').value = null;
  useCookie('entry-today').value = null;


  clearNuxtData();
  clearNuxtState();

  reloadNuxtApp({ path: '/', force: true });
}

const clear = () => clearError({ redirect: '/' });

</script>
