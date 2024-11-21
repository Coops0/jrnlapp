<template>
  <div class="flex flex-col w-full h-full bg-colors-primary-900 overflow-hidden">
    <div class="absolute inset-0 overflow-hidden">
      <div class="absolute -inset-[10px] opacity-50">
        <div
            class="absolute top-1/4 left-1/4 w-96 h-96 bg-colors-accent-400/20 rounded-full blur-3xl animate-pulse"></div>
        <div
            class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-colors-secondary-600/20 rounded-full blur-3xl animate-pulse delay-700"></div>
      </div>
    </div>

    <main class="relative flex flex-col flex-grow items-center justify-center px-4 py-6 md:py-16">
      <div class="relative space-y-16 max-w-3xl w-full">
        <div class="space-y-8 text-center">
          <div class="space-y-4">
            <div class="flex items-center justify-center space-x-3">
              <div class="w-3 h-3 rounded-full bg-colors-accent-400 animate-pulse"></div>
              <h1 class="text-7xl sm:text-9xl font-extralight tracking-tight text-colors-primary-100">
                jrnl.fm
              </h1>
              <div class="w-3 h-3 rounded-full bg-colors-secondary-600 animate-pulse delay-300"></div>
            </div>
            <p class="text-xl sm:text-2xl text-colors-primary-300 font-light tracking-wide">
              one day at a time
            </p>
          </div>

          <div class="flex flex-col sm:flex-row items-center justify-center gap-4 sm:gap-6">
            <NuxtLink
                class="relative inline-block text-lg group w-64"
                :to="!!jwt ? '/current' : '/login'"
            >
              <span
                  class="relative z-10 block px-5 py-5 overflow-hidden font-medium leading-tight text-colors-primary-100 transition-colors duration-300 ease-out border-2 border-colors-primary-200 rounded-lg group-hover:text-colors-primary-900">
                <span class="absolute inset-0 w-full h-full px-5 py-3 rounded-lg bg-colors-primary-800"></span>
                <span
                    class="absolute left-0 w-full h-48 transition-all duration-300 origin-top-right -rotate-90 -translate-x-full translate-y-16 bg-colors-primary-200 group-hover:-rotate-180 ease"></span>
                <span class="relative flex items-center justify-center">
                 {{ !!jwt ? 'open your journal' : 'login' }}
                  <svg class="w-5 h-5 ml-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/>
                  </svg>
                </span>
              </span>
              <span
                  class="absolute bottom-0 right-0 w-full h-12 -mb-1.5 -mr-1.5 transition-all duration-200 ease-linear bg-colors-primary-200 rounded-lg group-hover:mb-0 group-hover:mr-0"></span>
            </NuxtLink>
          </div>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-6 px-4">
          <div
              class="p-6 rounded-lg bg-colors-primary-800/40 backdrop-blur-md hover:bg-colors-primary-800/100 transition-all ease-out">
            <div class="text-colors-accent-400 mb-3">out of your way</div>
            <p class="text-colors-primary-300 text-sm">no ads. no subscriptions. no purchases. no popups.</p>
          </div>
          <div
              class="p-6 rounded-lg bg-colors-primary-800/40 backdrop-blur-md hover:bg-colors-primary-800/100 transition-all ease-out">
            <div class="text-colors-secondary-400 mb-3">locked</div>
            <p class="text-colors-primary-300 text-sm">type in your journal during the day. it locks once the day is
              over, and encrypts your entry.</p>
          </div>
          <div
              class="p-6 rounded-lg bg-colors-primary-800/40 backdrop-blur-md hover:bg-colors-primary-800/100 transition-all ease-out">
            <div class="text-colors-accent-400 mb-3">emotional rating</div>
            <p class="text-colors-primary-300 text-sm">use a slider, that's it.</p>
            <FormSlider :max="10" :min="1" :step="0.1" :model-value="5"/>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();

definePageMeta({ redirectUnautheticated: false });
</script>