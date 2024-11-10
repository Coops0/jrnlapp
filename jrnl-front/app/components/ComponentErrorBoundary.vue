<template>
  <NuxtErrorBoundary>
    <slot/>
    <template #error="{ error, clearError }">
      <div class="flex-grow size-full text-colors-text-200 items-center p-4">
        <div class="text-red-400 text-sm lowercase mb-4 text-center">{{ parseError(error) }}</div>

        <FormButton size="md" variant="secondary" full @click="clearError">
          ok
        </FormButton>
      </div>
    </template>
  </NuxtErrorBoundary>
</template>

<script lang="ts" setup>
interface ApiError {
  code: string;
  msg: string;
  status: number;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const parseError = (error: any): string => {
  try {
    if (typeof error === 'string') {
      const parsed = JSON.parse(error) as ApiError;
      return parsed.msg;
    }

    if (error.value) {
      return (error.value.data as ApiError).msg;
    }
  } catch {
    /* empty */
  }

  return error.toString();
};
</script>