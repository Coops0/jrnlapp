<template>
  <div class="flex-grow size-full text-colors-text-200 items-center p-4 justify-center">
    <div class="text-red-400 text-sm lowercase mb-4 text-center">
      <span v-if="error">{{ parsedError }}</span>
      <span v-else>an error occurred</span>
    </div>

    <FormButton v-if="clearError" size="md" variant="secondary" full @click="emit('clearError')">
      ok
    </FormButton>
    <slot/>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{ error: any }>();
const emit = defineEmits<{ clearError: [] }>();

interface ApiError {
  code: string;
  msg: string;
  status: number;
}

const parsedError = computed<string>(() => {
  try {
    if (typeof props.error === 'string') {
      const parsed = JSON.parse(props.error) as ApiError;
      return parsed.msg ?? 'an error occurred';
    }

    if (props.error.value) {
      return (props.error.value.data as ApiError).msg ?? 'an error occured';
    }
  } catch {
    /* empty */
  }

  return props.error.toString();
});
</script>