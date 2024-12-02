<template>
  <div class="relative perspective-lock blur-md">
    <div
        :class="{ 'animate-lock-enter': !messageTransition }"
        class="relative"
    >
      <div class="absolute left-1/2 top-0 -translate-x-1/2 w-12 h-16">
        <div class="w-full h-full border-4 border-b-0 border-blue-400 rounded-t-2xl" />
      </div>

      <div class="absolute top-12 left-1/2 -translate-x-1/2 w-20 h-20">
        <div class="w-full h-full border-4 border-blue-400 rounded-2xl bg-transparent">
          <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-4 h-8">
            <div class="w-full h-4 border-4 border-blue-400 rounded-full" />
            <div class="w-1 h-4 mx-auto bg-blue-400" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ messageTransition: boolean }>();
</script>

<style>
.perspective-lock {
  perspective: 1000px;
  transform-style: preserve-3d;
}

.animate-lock-enter {
  animation: lock-enter 1s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

@keyframes lock-enter {
  0% {
    transform: scale(0.5) rotateY(-180deg);
    opacity: 0;
  }
  50% {
    transform: scale(1.1) rotateY(0);
    opacity: 1;
  }
  75% {
    transform: scale(0.95) rotateY(0);
  }
  100% {
    transform: scale(1) rotateY(0);
    opacity: 1;
  }
}
</style>