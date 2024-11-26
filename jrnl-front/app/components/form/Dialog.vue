<template>
  <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition duration-200 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
  >
    <div v-if="isOpen"
         class="fixed inset-0 bg-colors-primary-950/90 backdrop-blur-sm flex items-center justify-center p-4 z-30">
      <div
          class="bg-colors-primary-900 rounded-xl w-full max-w-md shadow-xl border border-colors-primary-800 relative"
          @click.stop
      >
        <button
            v-if="showClose"
            class="absolute top-4 right-4 text-colors-primary-400 hover:text-colors-primary-200 transition-colors"
            @click="close"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>

        <div class="p-6">
          <slot :close="close"/>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script lang="ts" setup>
const isOpen = defineModel<boolean>({ required: true, default: false });
defineProps<{ showClose?: boolean }>();

const close = () => {
  isOpen.value = false;
};
</script>