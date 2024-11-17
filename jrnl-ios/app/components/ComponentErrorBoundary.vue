<template>
  <NuxtErrorBoundary>
    <slot/>
    <template #error="{ error, clearError }">
      <div class="flex-grow size-full text-colors-text-200 items-center p-4">
        <div class="text-red-400 text-sm lowercase mb-4 text-center">
          <span v-if="error">{{ parseError(error) }}</span>
          <span v-else>an error occurred</span>
        </div>

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
  console.log(error);
  try {
    if (typeof error === 'string') {
      const parsed = JSON.parse(error) as ApiError;
      return parsed.msg ?? 'an error occured';
    }
  } catch {
    /* empty */
  }

  try {
    if (error.value) {
      return (error.value.data as ApiError).msg ?? 'an error occured';
    }
  } catch {
    /* empty */
  }

  try {
    if (error.message) {
      return error.message;
    }
  } catch {
    /* empty */
  }

  try {
    if (error._value) {
      return error._value;
    }
  } catch {
    /* empty */
  }

  return error.toString();
};
</script>