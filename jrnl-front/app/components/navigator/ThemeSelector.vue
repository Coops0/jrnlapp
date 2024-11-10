<template>
  <div>
    <button
        class="text-colors-primary-500 hover:text-colors-primary-700 transition-colors"
        @click="() => isOpen = !isOpen"
    >
      t
    </button>

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
          class="absolute right-0 mb-2 bottom-5 w-40 z-10 rounded-lg bg-colors-primary-900/95 backdrop-blur-sm border border-colors-primary-700 shadow-xl"
      >
        <div class="p-3">
          <div class="grid grid-cols-2 gap-2">
            <button
                v-for="[name, colors] in themesWithoutPlaceholder"
                :key="name"
                :style="{ backgroundColor: colors.colors.primary[400] }"
                class="group relative flex items-center justify-center h-14 rounded-lg transition-all duration-200 hover:scale-105"
                :class="{ 'ring-2 ring-colors-accent-400': theme === name }"
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
                  class="absolute top-2 right-2 w-2 h-2 rounded-full bg-colors-accent-400"
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

const { $localApi } = useNuxtApp();
const userService = new UserService($localApi);
const { theme, setTheme } = useTheme(userService);

// todo close if click outside
const isOpen = ref(false);

const themesWithoutPlaceholder = computed(() =>
    Object.entries(themes).filter(([name]) => name !== 'lunar_placeholder')
);

function selectTheme(name: string) {
  isOpen.value = false;
  setTheme(name);
}
</script>