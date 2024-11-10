<template>
  <section class="space-y-4">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <h2 class="text-lg md:text-xl font-light text-colors-primary-200">moods by week</h2>
      <div class="flex items-center gap-2">
        <FormButton
            size="sm"
            variant="ghost"
            @click="() => emit('move', false)"
        >
          <span class="text-colors-primary-400 text-lg">{{ '<' }}</span>
        </FormButton>
        <span class="text-sm md:text-base text-colors-primary-400 min-w-[140px] text-center">
             <NuxtTime
                 :datetime="addDays(before, 6)"
                 day="2-digit"
                 month="numeric"
             /> - <NuxtTime
            :datetime="before"
            day="2-digit"
            month="numeric"/>
        </span>
        <FormButton
            size="sm"
            variant="ghost"
            @click="() => emit('move', true)"
        >
          <span class="text-colors-primary-400 text-lg">{{ '>' }}</span>
        </FormButton>
      </div>
    </div>

    <div class="relative h-64 md:h-96 rounded-lg overflow-hidden">
      <div class="absolute inset-x-8 bottom-0 flex justify-between">
        <div
            v-for="day in WEEK_DAYS"
            :key="day"
            class="flex flex-col items-center w-full"
        >
          <div class="h-full w-px border-l border-colors-primary-800/30"/>
          <span class="text-xs text-colors-primary-500 mt-2">{{ day }}</span>
        </div>
      </div>

      <div
          class="absolute inset-y-4 left-4 flex flex-col justify-between text-xs md:text-sm text-colors-primary-500">
        <span>10</span>
        <span>5</span>
        <span>0</span>
      </div>

      <div
          v-for="dayGroup in days"
          :key="dayGroup.day"
          :style="{
            left: getDayPosition(parseServerDate(dayGroup.day)) + '%'
          }"
          class="absolute w-1/7 h-full transition-all duration-300"
      >
        <div
            v-for="(scale, idx) in dayGroup.scales"
            :key="`${dayGroup.day}-${idx}`"
            :style="{
              backgroundColor: ratingLerp(scale, theme),
              bottom: ((scale / 10) * 100) + '%',
              left: '50%',
              transform: 'translateX(-50%)',
              opacity: 0.8
            }"
            :title="scale.toString()"
            class="absolute w-3 md:w-4 h-3 md:h-4 rounded-full transition-all duration-300"
        >
          <div
              class="absolute inset-0 rounded-full hover:ring-2 ring-colors-primary-200 transition-all"
          />
        </div>
      </div>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type { GroupedDayData } from '~/types/weekly-data.type';
import { addDays, parseServerDate, ratingLerp } from '~/util/index.util';

defineProps<{
  days: GroupedDayData[];
  theme: string;
  before: Date;
}>();

const emit = defineEmits<{
  move: [foward: boolean];
}>();

const WEEK_DAYS = ['sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'] as const;
const getDayPosition = (date: Date) => (((date.getDay() + 1) / 7) * 100) - 6;
</script>
