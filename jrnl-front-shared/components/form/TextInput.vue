<template>
  <input
      v-model="model"
      :disabled="disabled"
      :placeholder="placeholder"
      :readonly="readonly"
      class="w-full px-3 py-2 rounded-md bg-colors-primary-800/60 text-colors-primary-100 placeholder:text-colors-primary-600 border border-colors-primary-700 focus:border-colors-primary-500 outline-none"
      type="text"
      v-bind="$attrs"
      @input="() => emit('input', model)"
  >
</template>

<script lang="ts" setup>
const model = defineModel<string>({ default: '' });

interface Props {
  placeholder?: string;
  disabled?: boolean;
  readonly?: boolean;
}

withDefaults(defineProps<Props>(), {
  placeholder: '',
  disabled: false,
  readonly: false
});

const emit = defineEmits<{
  input: [value: string]
}>();
</script>

