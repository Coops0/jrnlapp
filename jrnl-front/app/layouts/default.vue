<template>
  <div class="min-h-screen flex flex-col bg-colors-primary-900">
    <header class="md:block hidden">
      <NavigatorDesktop v-if="authenticated" :name="user?.name"/>
    </header>

    <main class="flex-grow flex w-full mx-auto px-4 py-6 mb-0">
      <ComponentErrorBoundary>
        <slot/>
      </ComponentErrorBoundary>
    </main>

    <NavigatorMobile v-if="authenticated" class="md:hidden"/>

    <footer class="hidden md:block">
      <div class="max-w-5xl mx-auto px-4 py-4 flex ml-4">
        <p class="text-sm text-colors-primary-400">jrnl.fm</p>
      </div>
    </footer>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();
const { user } = useUser(null);

const authenticated = computed(() => !!jwt.value?.length);
</script>