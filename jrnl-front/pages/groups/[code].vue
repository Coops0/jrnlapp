<template>
  <div class="w-full max-w-6xl mx-auto px-4 space-y-8">
    <div v-if="group" class="space-y-2">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl md:text-3xl font-light text-colors-primary-100">{{ group.name }}</h1>
          <p class="text-sm md:text-base text-colors-primary-400 font-bold">
            {{ isOwned ? 'you own this group' : '' }}
          </p>
        </div>

        <button
            class="px-4 py-2 rounded-md text-sm md:text-base text-colors-accent-400 hover:bg-colors-primary-800/60 transition-colors"
            @click="leave"
        >
          leave
        </button>
      </div>
    </div>

    <div v-else class="h-12 md:h-16 bg-colors-primary-900/40 rounded-lg"/>

    <section v-if="days" class="space-y-4">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <h2 class="text-lg md:text-xl font-light text-colors-primary-200">moods by week</h2>
        <div class="flex items-center gap-2">
          <button
              class="p-2 rounded-lg hover:bg-colors-primary-800/60 transition-colors cursor-pointer"
              @click="() => move(false)"
          >
            <span class="text-colors-primary-400 text-lg">←</span>
          </button>
          <span
              class="text-sm md:text-base text-colors-primary-400 min-w-[140px] text-center"
              data-allow-mismatch="text">
            {{ dateRangeString }}
          </span>
          <button
              class="p-2 rounded-lg hover:bg-colors-primary-800/60 transition-colors cursor-pointer"
              @click="() => move(true)"
          >
            <span class="text-colors-primary-400 text-lg">→</span>
          </button>
        </div>
      </div>

      <div class="relative h-64 md:h-96 bg-colors-primary-900/40 rounded-lg overflow-hidden">
        <div class="absolute inset-x-8 bottom-0 flex justify-between">
          <div
              v-for="(day, index) in WEEK_DAYS"
              :key="index"
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
            v-for="day in days"
            :key="day.day"
            :style="{
            left: `${getDayPosition(parseServerDate(day.day))}%`,
          }"
            class="absolute w-1/7 h-full transition-all duration-300"
        >
          <div
              v-for="(scale, idx) in day.scales"
              :key="`${day.day}-${idx}`"
              :style="{
              backgroundColor: ratingLerp(scale, theme),
              bottom: `${(scale / 10) * 100}%`,
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
    <div v-else class="h-64 md:h-96 bg-colors-primary-900/40 rounded-lg"/>

    <section v-if="members" class="space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-lg md:text-xl font-light text-colors-primary-200">people</h2>
        <span class="text-sm md:text-base text-colors-primary-400">{{ members.length }} people</span>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
        <div
            v-for="(member, index) in members"
            :key="member.id"
            :class="{
              'border-2 border-colors-accent-400': member.id === user?.id,
              'border-2 border-colors-primary-700': member.owner,
            }"
            class="flex items-center justify-between p-3 md:p-4 rounded-lg bg-colors-primary-900/40"
        >
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 md:w-10 md:h-10 rounded-full bg-colors-primary-700 flex items-center justify-center">
              <span class="text-sm md:text-base text-colors-primary-200">
                {{ member.name[0].toUpperCase() }}
              </span>
            </div>
            <span class="text-colors-primary-200 md:text-lg">{{ member.name.toLowerCase() }}</span>
          </div>

          <button
              v-if="isOwned && member.id !== user?.id"
              class="px-3 py-1 rounded text-sm md:text-base text-colors-accent-400 hover:bg-colors-primary-800/60 transition-colors"
              @click="kick(index)"
          >
            kick
          </button>
        </div>
      </div>
    </section>
    <div v-else class="h-48 bg-colors-primary-900/40 rounded-lg"/>
  </div>
</template>

<script lang="ts" setup>
import { GroupService } from '~/services/group.service';
import { addDays, parseServerDate, ratingLerp } from '~/util/index.util';
import { UserService } from '~/services/user.service';

const route = useRoute();
const code = route.params.code as string;

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);
const userService = new UserService($localApi);

const { theme } = useTheme(null);
const { user } = useUser(userService);
const { group, members, days, before } = useGroup(code, groupService);

const WEEK_DAYS = ['sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'] as const;

const isOwned = computed(() => members.value?.some(m => m.owner && m.id === user.value?.id));

const getDayPosition = (date: Date) => (date.getDay() / 6) * 100;

function move(forward: boolean) {
  // I don't even know
  before.value = addDays(before.value, forward ? -7 : 7);
}

const dateRangeString = computed(() => {
  const end = before.value;
  const start = addDays(end, -6);

  const s = start.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  const e = end.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  return `${s} - ${e}`;
});

async function kick(index: number) {
  const member = members.value!.splice(index, 1)[0]!;

  try {
    await groupService.kickMember(code, member.id);
  } catch (e) {
    console.error(e);
    members.value!.splice(index, 0, member);
  }
}

async function leave() {
  try {
    await groupService.leaveGroup(code);
    await navigateTo('/groups/');
  } catch (e) {
    console.error(e);
  }
}
</script>