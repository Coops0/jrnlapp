<template>
  <section class="p-4 sm:p-6 rounded-xl bg-colors-primary-900/30 space-y-6">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div class="flex items-center gap-3 ml-auto">
        <FormButton size="sm" variant="ghost" @click="() => emit('move', false)">
          <span class="text-colors-primary-400 text-lg">←</span>
        </FormButton>

        <span class="text-sm md:text-base text-colors-primary-400 min-w-[140px] text-center">
          <NuxtTime :datetime="addDays(before, 6)" day="2-digit" month="numeric"/>
          -
          <NuxtTime :datetime="before" day="2-digit" month="numeric"/>
        </span>

        <FormButton size="sm" variant="ghost" @click="() => emit('move', true)">
          <span class="text-colors-primary-400 text-lg">→</span>
        </FormButton>
      </div>
    </div>

    <div class="grid grid-cols-7 gap-2 sm:gap-4 h-64 sm:h-80">
      <div v-for="(dayGroup, index) in days" :key="dayGroup.day.getTime()" class="relative flex flex-col">
        <div class="flex-1 bg-colors-primary-950/50 rounded-lg relative">
          <div class="absolute inset-x-0 bottom-0 flex flex-col-reverse gap-1 p-2">
            <div
                v-for="scale in dayGroup.scales"
                :key="scale"
                class="w-full aspect-square rounded-lg cursor-pointer transition-all duration-300
                     hover:ring-2 ring-colors-primary-200 group relative"
                :style="{
                backgroundColor: ratingLerp(scale, theme),
                opacity: 0.8
              }"
            >
              <div
                  class="absolute opacity-0 group-hover:opacity-100 transition-opacity
                          -top-8 left-1/2 -translate-x-1/2 bg-colors-primary-800
                          text-colors-primary-200 text-xs rounded px-2 py-1 whitespace-nowrap">
                mood ~ {{ scale }}/10
              </div>
            </div>
          </div>

          <div
              class="absolute -left-3 top-0 h-full flex flex-col justify-between pointer-events-none"
              :class="index === 0 ? 'visible' : 'invisible'">
            <span v-for="n in 5" :key="n" class="text-xs text-colors-primary-500">
              {{ (6 - n) * 2 }}
            </span>
          </div>
        </div>

        <span class="mt-2 text-xs text-center text-colors-primary-500">
          {{ WEEK_DAYS[index] }}
        </span>
      </div>
    </div>
  </section>
</template>

<script lang="ts" setup>
import { addDays, ratingLerp } from '~/util/index.util';

defineProps<{
  days: { day: Date; scales: number[]; }[];
  theme: string;
  before: Date;
}>();

const emit = defineEmits<{
  move: [foward: boolean];
}>();

const WEEK_DAYS = ['sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'] as const;
</script>