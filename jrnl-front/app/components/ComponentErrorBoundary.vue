<template>
  <NuxtErrorBoundary>
    <slot/>
    <template #error="{ error, clearError }">
      <div class="flex-grow size-full bg-colors-background-900 text-colors-text-200 items-center p-4">
        <div class="text-red-400 text-sm lowercase mb-4 text-center">{{ error }}</div>

        <FormButton size="md" variant="secondary" full @click="clearError">
          clear error
        </FormButton>
      </div>
    </template>
  </NuxtErrorBoundary>
</template>