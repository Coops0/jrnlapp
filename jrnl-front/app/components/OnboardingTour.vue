<template>
  <div v-if="goodToGo">
    <component :is="goodToGo" :tour-id="1" ref="onboardingTourInstance" :steps/>
  </div>
</template>

<script setup lang="ts">
// @ts-expect-error plugin is not typed
// noinspection TypeScriptCheckImport
import vot from 'vue-onboarding-tour';
import type { Component, Plugin } from 'vue';

const goodToGo = shallowRef<Component | null>(null);
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const onboardingTourInstance = ref<any>(null);

onMounted(() => {
  const { vueApp } = useNuxtApp();

  const mockVueApp = {
    use(plugin: Plugin) {
      vueApp.use(plugin);
    },
    component(name: string, component: Component) {
      goodToGo.value = component;
    }
  };

  try {
    vot.install(mockVueApp);
  } catch (e) {
    console.error('could not load VueOnboardingTour', e);
    return;
  }

  setTimeout(() => onboardingTourInstance.value.startTour());
});

const steps = [
  {
    target: '#jrnl-logo',
    title: 'welcome to jrnl',
    description: 'click or hold the logo down to navigate thru jrnl',
    tag: 'Navigation'
  },
  {
    target: '#last-saved',
    title: 'save status',
    description: 'click this indicator to toggle your view of the last time your jrnl was saved',
    tag: 'Saving'
  },
  {
    target: '#tomorrow-lock',
    title: 'time until tmrw',
    description: 'this indicator displays how long you have until your jrnl for the day is locked',
    tag: 'Countdown'
  },
  {
    target: '#ephemeral-mode',
    title: 'ephemeral mode',
    description: 'ephemeral mode deletes today\'s entry instead of saving it when the day is over',
    tag: 'Ephemeral'
  },
  {
    target: '#emotion-slider',
    title: 'scale',
    description: 'use this slider to rate your day, your mood, whatever you want',
    tag: 'Slider'
  }
];
</script>