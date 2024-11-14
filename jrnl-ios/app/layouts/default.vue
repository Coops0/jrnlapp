<template>
  <div class="size-full flex flex-col my-8">
    <main class="flex-grow flex size-full mx-auto px-4 py-6 mb-0">
      <ComponentErrorBoundary>
        <slot/>
      </ComponentErrorBoundary>
    </main>

    <footer class="w-full">
      <div class="mx-auto px-4 py-3 w-full">
        <NavigatorLogo/>
      </div>
    </footer>
  </div>
</template>