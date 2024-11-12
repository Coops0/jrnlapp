<template>
  <div class="flex flex-col w-full h-full overflow-hidden cosmic-wave">
    <div class="absolute inset-0 overflow-hidden">
      <div class="absolute -inset-[10px] opacity-10">
        <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-purple-500/5 rounded-full blur-3xl animate-pulse"></div>
        <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-indigo-500/5 rounded-full blur-3xl animate-pulse delay-700"></div>
      </div>
    </div>

    <main class="relative flex flex-col flex-grow items-center justify-center px-4 py-6 md:py-16">
      <div class="relative space-y-16 max-w-3xl w-full">
        <div class="space-y-8 text-center">
          <div class="space-y-4">
            <div class="flex items-center justify-center space-x-3">
              <div class="w-2 h-2 rounded-full bg-purple-400/20 animate-pulse"></div>
              <h1 class="text-7xl sm:text-9xl font-extralight tracking-tight text-neutral-200/60">
                jrnl.fm
              </h1>
              <div class="w-2 h-2 rounded-full bg-purple-400/20 animate-pulse delay-300"></div>
            </div>
            <p class="text-xl sm:text-2xl text-neutral-400/60 font-light tracking-wide">
              one day at a time
            </p>
          </div>

          <div class="flex flex-col sm:flex-row items-center justify-center gap-4 sm:gap-6">
            <NuxtLink
                class="relative inline-block text-lg group w-64"
                :to="!!jwt ? '/current' : '/login'"
            >
              <span class="relative z-10 block px-5 py-5 overflow-hidden font-medium leading-tight text-neutral-300/80 transition-colors duration-300 ease-out border-2 border-neutral-500/10 rounded-lg group-hover:text-neutral-200">
                <span class="absolute inset-0 w-full h-full px-5 py-3 rounded-lg bg-neutral-900/50"></span>
                <span class="absolute left-0 w-full h-48 transition-all duration-300 origin-top-right -rotate-90 -translate-x-full translate-y-16 bg-purple-800/10 group-hover:-rotate-180 ease"></span>
                <span class="relative flex items-center justify-center">
                  {{ !!jwt ? 'open your journal' : 'login' }}
                  <svg class="w-5 h-5 ml-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M13 7l5 5m0 0l-5 5m5-5H6"/>
                  </svg>
                </span>
              </span>
              <span class="absolute bottom-0 right-0 w-full h-12 -mb-1.5 -mr-1.5 transition-all duration-200 ease-linear bg-neutral-500/10 rounded-lg group-hover:mb-0 group-hover:mr-0 group-hover:opacity-0"></span>
            </NuxtLink>
          </div>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-6 px-4">
          <div class="p-6 rounded-lg bg-neutral-900/30 backdrop-blur-sm hover:bg-neutral-800/30 transition-all ease-out border border-neutral-800/10">
            <div class="text-purple-300/40 mb-3">out of your way</div>
            <p class="text-neutral-400/60 text-sm">no ads. no subscriptions. no purchases. no popups.</p>
          </div>
          <div class="p-6 rounded-lg bg-neutral-900/30 backdrop-blur-sm hover:bg-neutral-800/30 transition-all ease-out border border-neutral-800/10">
            <div class="text-purple-300/40 mb-3">locked</div>
            <p class="text-neutral-400/60 text-sm">type in your journal during the day. it locks once the day is over, and encrypts your entry.</p>
          </div>
          <div class="p-6 rounded-lg bg-neutral-900/30 backdrop-blur-sm hover:bg-neutral-800/30 transition-all ease-out border border-neutral-800/10">
            <div class="text-purple-300/40 mb-3">emotional rating</div>
            <p class="text-neutral-400/60 text-sm">use a slider, that's it.</p>
            <FormSlider :max="10" :min="1" :step="0.1" :rating-lerp="() => '#a1a1aa'" :model-value="5"/>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script lang="ts" setup>
const { jwt } = useAuth();

definePageMeta({
  redirectUnautheticated: false,
  layout: false
});
</script>

<!--suppress CssReplaceWithShorthandSafely, CssRedundantUnit -->
<style>
.cosmic-wave {
  animation: cosmic-wave 4.5s ease infinite;
  background-blend-mode: color-dodge, color-burn, color-burn, normal;
}

@keyframes cosmic-wave {
  0% {
    background: radial-gradient(100% 225% at 0 100%, #0a0a0a 0, #121212 100%),
    linear-gradient(-45deg, #1a1a1a 70%, #121212 100%),
    linear-gradient(135deg, #080808 0, #0a0a0a 100%),
    linear-gradient(60deg, #121212 0, #0a0a0a 50%);
    background-size: 100% 100%;
    background-position: 0% 60%;
  }
  20% {
    background: radial-gradient(100% 225% at 0 100%, #0a0a0a 0, #121212 100%),
    linear-gradient(-45deg, #1a1a1a 70%, #121212 100%),
    linear-gradient(135deg, #080808 0, #0a0a0a 100%),
    linear-gradient(60deg, #121212 0, #0a0a0a 50%);
    background-size: 105% 105%;
    background-position: 10% 50%;
  }
  40% {
    background: radial-gradient(100% 225% at 0 100%, #0a0a0a 0, #121212 100%),
    linear-gradient(-45deg, #1a1a1a 70%, #121212 100%),
    linear-gradient(135deg, #080808 0, #0a0a0a 100%),
    linear-gradient(60deg, #121212 0, #0a0a0a 50%);
    background-size: 110% 300%;
    background-position: 20% 40%;
  }
  60% {
    background: radial-gradient(100% 225% at 0 100%, #0a0a0a 0, #121212 100%),
    linear-gradient(-45deg, #1a1a1a 70%, #121212 100%),
    linear-gradient(135deg, #080808 0, #0a0a0a 100%),
    linear-gradient(60deg, #121212 0, #0a0a0a 50%);
    background-size: 115% 115%;
    background-position: 30% 30%;
  }
  80% {
    background: radial-gradient(100% 225% at 0 100%, #0a0a0a 0, #121212 100%),
    linear-gradient(-45deg, #1a1a1a 70%, #121212 100%),
    linear-gradient(135deg, #080808 0, #0a0a0a 100%),
    linear-gradient(60deg, #121212 0, #0a0a0a 50%);
    background-size: 250% 120%;
    background-position: 40% 20%;
  }
  100% {
    background: radial-gradient(100% 225% at 0 100%, #0a0a0a 0, #121212 100%),
    linear-gradient(-45deg, #1a1a1a 70%, #121212 100%),
    linear-gradient(135deg, #080808 0, #0a0a0a 100%),
    linear-gradient(60deg, #121212 0, #0a0a0a 50%);
    background-size: 100% 100%;
    background-position: 0% 60%;
  }
}
</style>