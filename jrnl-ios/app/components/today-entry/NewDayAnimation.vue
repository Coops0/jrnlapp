<template>
  <Transition
      appear
      @after-enter="startMessageTransition"
      @after-leave="() => emit('transition-complete')"
  >
    <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-colors-primary-950/90 backdrop-blur-sm transition-all duration-1000"/>

      <div class="relative flex flex-col items-center">
        <TodayEntryLockIcon :message-transition="messageTransition" />

        <Transition
            name="message"
            @after-enter="startExitSequence"
        >
          <div
              v-if="messageVisible"
              class="mt-8 text-2xl font-medium text-colors-primary-200"
          >
            <Transition name="message-swap" mode="out-in">
              <div v-if="!messageTransition" key="locked">
                your journal is now locked
              </div>
              <div v-else key="new-day">
                starting a new day...
              </div>
            </Transition>
          </div>
        </Transition>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
const props = defineProps<{ show: boolean }>();

const emit = defineEmits<{ 'transition-complete': [] }>();

const messageVisible = ref(false);
const messageTransition = ref(false);

const startMessageTransition = () => {
  messageVisible.value = true;
};

const startExitSequence = () => {
  setTimeout(() => {
    messageTransition.value = true;
    setTimeout(() => {
      emit('transition-complete');
    }, 1000);
  }, 1500);
};

watch(() => props.show, s => {
  if (!s) {
    messageVisible.value = false;
    messageTransition.value = false;
  }
});
</script>

<!--suppress CssUnusedSymbol -->
<style>
.message-enter-active,
.message-leave-active {
  transition: opacity 0.5s ease, transform 0.5s ease;
}

.message-enter-from,
.message-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

.message-swap-enter-active,
.message-swap-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.message-swap-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.message-swap-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>