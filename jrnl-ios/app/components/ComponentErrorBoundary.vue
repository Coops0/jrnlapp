<template>
  <NuxtErrorBoundary>
    <slot/>
    <template #error="{ error, clearError }">
      <ErrorDisplay :error="error" @clear-error="clearError"/>
    </template>
  </NuxtErrorBoundary>
</template>