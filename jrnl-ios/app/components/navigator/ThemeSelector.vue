<template>
  <div>
    <Transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="transform scale-95 opacity-0"
        enter-to-class="transform scale-100 opacity-100"
        leave-active-class="transition duration-150 ease-in"
        leave-from-class="transform scale-100 opacity-100"
        leave-to-class="transform scale-95 opacity-0"
    >
      <div
          v-if="isOpen"
          ref="popupWindow"
          class="absolute w-40 z-20 rounded-lg backdrop-blur-xl shadow-xl outline-none"
          :style="{ top: y + 'px', left: x + 'px' }"
          tabindex="-1"
          @focusout="unfocus"
      >
        <div class="p-3">
          <div class="grid grid-cols-2 gap-2">
            <button
                v-for="[name, colors] in themesWithoutPlaceholder"
                :key="name"
                :title="name"
                :style="{ backgroundColor: colors.colors.primary[400] }"
                class="group relative flex items-center justify-center h-14 rounded-lg transition-all duration-200 hover:scale-105"
                :class="{ 'ring-2 ring-colors-secondary-400': theme === name }"
                @click="() => selectTheme(name)"
            >
              <span
                  :style="{ color: colors.colors.text[50] }"
                  class="text-sm font-medium opacity-0 group-hover:opacity-100 transition-opacity"
              >
                {{ name }}
              </span>

              <span
                  v-if="theme === name"
                  class="absolute top-2 right-2 w-2 h-2 rounded-full bg-colors-secondary-600"
              />
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { UserService } from '~/services/user.service';
import { themes } from '~/assets/themes';

const isOpen = defineModel<boolean>({ default: false });
withDefaults(defineProps<{
  x?: number;
  y?: number;
}>(), {
  x: 5,
  y: 5
});

const { $localApi } = useNuxtApp();
const userService = new UserService($localApi);
const { theme, setTheme } = await useTheme(userService);

const popupWindow = ref<HTMLElement | null>(null);

const themesWithoutPlaceholder = computed(() =>
    Object.entries(themes).filter(([name]) => name !== 'lunar_placeholder')
);

async function selectTheme(name: string) {
  isOpen.value = false;
  await setTheme(name);
}

const focusNextFrame = ref(false);

watch(isOpen, o => {
  if (o) {
    focusNextFrame.value = true;
  }
}, { immediate: true });

watch(popupWindow, (el) => {
  if (el && focusNextFrame.value) {
    el.focus();
    focusNextFrame.value = false;
  }
}, { immediate: true });


function unfocus() {
  isOpen.value = false;
}
</script>