<template>
  <div class="min-h-screen flex flex-col bg-colors-primary-900">
    <header class="sticky top-0 backdrop-blur-sm bg-colors-primary-900/90 border-b border-colors-primary-700">
      <nav class="max-w-5xl mx-auto px-4 py-3 flex items-center justify-between">
        <NuxtLink
            :to="jwt?.length ? '/page' : '/'"
            class="text-2xl font-semibold text-colors-primary-100 hover:text-colors-primary-300 transition-colors"
        >
          jrnl
        </NuxtLink>

        <div class="flex items-center gap-6">
          <template v-if="!jwt?.length">
            <NuxtLink
                to="/login"
                class="px-4 py-2 rounded-full bg-colors-primary-800 text-colors-primary-100 hover:bg-colors-primary-700 transition-colors"
            >
              login
            </NuxtLink>
          </template>
          <template v-else>
            <p v-if="user?.name" class="text-lg font-thin text-colors-primary-200 lowercase">
              {{ user?.name }}
            </p>
            <NuxtLink
                to="/pages"
                class="text-colors-primary-400 hover:text-colors-primary-200 transition-colors"
            >
              pages
            </NuxtLink>
            <NuxtLink
                to="/groups"
                class="text-colors-primary-400 hover:text-colors-primary-200 transition-colors"
            >
              groups
            </NuxtLink>
            <NuxtLink
                to="/color"
                class="text-colors-primary-400 hover:text-colors-primary-200 transition-colors"
            >
              theme
            </NuxtLink>
            <NuxtLink
                to="/logout"
                class="text-colors-primary-400 hover:text-colors-primary-200 transition-colors"
            >
              logout
            </NuxtLink>
          </template>
        </div>
      </nav>
    </header>

    <main class="flex-grow max-w-5xl w-full mx-auto px-4 py-6">
      <slot/>
    </main>

    <footer class="border-t border-colors-primary-700 bg-colors-primary-900/50">
      <div class="max-w-5xl mx-auto px-4 py-4 flex items-center justify-between">
        <p class="text-sm text-colors-primary-400">jrnl app / cooper hanessian & yonald chery</p>
      </div>
    </footer>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();
const { user } = useUser(null);
</script>