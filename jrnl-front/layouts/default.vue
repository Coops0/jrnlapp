<template>
  <div class="min-h-screen flex flex-col bg-colors-primary-900">
    <header class="md:block hidden">
      <NavigatorDesktop v-if="authenticated" :authenticated :name="user?.name"/>
    </header>

    <main class="flex-grow flex w-full mx-auto px-4 py-6 mb-0">
      <slot/>
    </main>

    <NavigatorMobile v-if="authenticated" :authenticated class="md:hidden"/>

    <footer class="bg-colors-primary-900/50 hidden md:block">
      <div class="max-w-5xl mx-auto px-4 py-4 flex items-center justify-between">
        <p class="text-sm text-colors-primary-400">jrnl app 2024 :)</p>
      </div>
    </footer>
  </div>
</template>

<script lang="ts" setup>

const { jwt } = useAuth();
const { user } = useUser(null);

const authenticated = computed(() => !!jwt.value?.length);
</script>