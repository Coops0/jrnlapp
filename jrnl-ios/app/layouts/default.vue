<template>
  <div class="size-full flex flex-col">
    <main class="flex-grow flex size-full mx-auto px-4 mb-0">
      <ComponentErrorBoundary>
        <slot/>
      </ComponentErrorBoundary>
    </main>

    <NavigatorLogo/>
  </div>
</template>