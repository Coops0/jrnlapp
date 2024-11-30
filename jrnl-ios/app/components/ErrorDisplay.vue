<template>
  <div class="flex-grow items-center p-4 justify-center">
    <div class="text-red-400 text-sm lowercase mb-4 text-center">
      <span v-if="error">{{ parsedError }}</span>
      <span v-else>an error occurred</span>
    </div>

    <div class="flex flex-row gap-2">
      <FormButton size="md" variant="secondary" full @click="emit('clear-error')">
        ok
      </FormButton>
      <slot/>
    </div>
  </div>
</template>

<script lang="ts" setup>
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const props = defineProps<{ error: any }>();
const emit = defineEmits<{ 'clear-error': [] }>();

interface ApiError {
  code: string;
  msg: string;
  status: number;
}

onMounted(() => {
  console.error(props.error);
});

const parsedError = computed<string>(() => {
  try {
    if (typeof props.error === 'string') {
      const parsed = JSON.parse(props.error) as ApiError;
      return parsed.msg ?? 'an error occurred';
    }
  } catch {
    /* empty */
  }

  try {
    if (props.error.value) {
      return (props.error.value.data as ApiError).msg ?? 'an error occurred';
    }
  } catch {
    /* empty */
  }

  try {
    if (typeof props.error?.value === 'string') {
      return props.error.value;
    }
  } catch {
    /* empty */
  }

  try {
    if (props.error.value?.cause?.message) {
      return props.error.value.cause.message;
    }
  } catch {
    /* empty */
  }

  try {
    if(props.error.value?.message) {
      return props.error.value.message;
    }
  } catch {
    /* empty */
  }

  return props.error.toString();
});
</script>