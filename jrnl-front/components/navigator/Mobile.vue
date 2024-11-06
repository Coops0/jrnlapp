<template>
  <div
      class="fixed bottom-0 left-0 right-0 transform transition-transform duration-300"
      :class="isHidden ? 'translate-y-[45%]' : 'translate-y-0'"
      @click="toggleNav"
  >
    <div class="bg-colors-primary-900/95 backdrop-blur-sm">
      <div class="flex items-center justify-between px-4 py-2">
        <button class="text-colors-primary-400 hover:text-colors-primary-200" ref="hideButton">
          {{ isHidden ? '' : 'v' }}
        </button>
        <Navigator :authenticated/>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import Navigator from '~/components/navigator/Navigator.vue';

defineProps<{ authenticated: boolean }>();

const isHidden = useCookie('mobile-nav-hidden', {
  default() {
    return false;
  }
});

const hideButton = ref<HTMLButtonElement | null>(null);

function toggleNav(e: MouseEvent) {
  if (!isHidden.value) {
    if (e.target !== hideButton.value) {
      return;
    }

    isHidden.value = true;
  } else {
    e.preventDefault();
    isHidden.value = false;
  }
}
</script>