<template>
  <div class="min-h-screen flex flex-col bg-colors-primary-900">
    <header class="w-full">
        <div v-if="authenticated" class="mx-auto px-4 py-3 w-full">
          <NavigatorLogo/>
        </div>
    </header>

    <main class="flex-grow flex w-full mx-auto px-4 py-6 mb-0">
      <ComponentErrorBoundary>
        <slot/>
      </ComponentErrorBoundary>
    </main>

    <footer class="hidden md:block">
      <div class="mx-auto px-4 py-4 flex ml-4">
        <p class="text-sm text-colors-primary-400">jrnl.fm</p>
      </div>
    </footer>
  </div>
</template>

<script lang="ts" setup>

const { jwt } = useAuth();
const authenticated = computed(() => !!jwt.value?.length);
</script>