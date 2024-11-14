<template>
  <button
      :class="[
      'transition-all duration-200 rounded-lg font-medium',
      'disabled:opacity-50 disabled:cursor-not-allowed',
      sizeClasses[size],
      variantClasses[variant],
      { 'w-full': full },
      className
    ]"
      :disabled
      :title
      :type="type"
      v-bind="$attrs"
  >

    <span>
      <slot/>
    </span>
  </button>
</template>

<script lang="ts" setup>
type ButtonVariant = 'primary' | 'secondary' | 'danger' | 'ghost' | 'link';
type ButtonSize = 'sm' | 'md' | 'lg';

interface Props {
  variant?: ButtonVariant;
  size?: ButtonSize;
  full?: boolean;
  type?: 'button' | 'submit' | 'reset';
  className?: string;
  disabled?: boolean;
  title?: string;
}

withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  full: false,
  type: 'button',
  className: '',
  disabled: false,
  title: '',
});

const sizeClasses = {
  sm: 'px-3 py-1 text-sm',
  md: 'px-4 py-2 text-base',
  lg: 'px-6 py-3 text-lg'
};

const variantClasses = {
  primary: 'bg-colors-primary-700 hover:bg-colors-primary-600 text-colors-primary-100',
  secondary: 'bg-colors-primary-900/40 hover:bg-colors-primary-800/60 text-colors-primary-400',
  danger: 'bg-colors-primary-900/40 hover:bg-colors-primary-800/60 text-colors-accent-400',
  ghost: 'hover:bg-colors-primary-800/60 text-colors-primary-400',
  link: 'hover:text-colors-primary-300 text-colors-primary-400 p-0'
};
</script>