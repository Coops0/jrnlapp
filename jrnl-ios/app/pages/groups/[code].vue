<template>
  <div class="w-full max-w-6xl mx-auto px-4 space-y-8">
    <ComponentErrorBoundary>
      <GroupManage v-if="group" :group :is-owned="isOwned ?? false" @leave="leave"/>
      <div v-else class="h-12 md:h-16 rounded-lg"/>
    </ComponentErrorBoundary>

    <ComponentErrorBoundary>
      <MoodsByWeek v-if="days" :before :days :theme @move="move"/>
      <div v-else class="h-64 md:h-96 rounded-lg"/>
    </ComponentErrorBoundary>

    <ComponentErrorBoundary>
      <GroupMembers v-if="members" :id="user?.id ?? ''" :is-owned="isOwned" :members="members" @kick="kick"/>
      <div v-else class="h-48 rounded-lg"/>
    </ComponentErrorBoundary>
  </div>
</template>

<script lang="ts" setup>
import { GroupService } from '~/services/group.service';
import { addDays } from '~/util/index.util';
import { UserService } from '~/services/user.service';
import MoodsByWeek from '~/components/group/MoodsByWeek.vue';
import { watchErrorAndThrow } from '~/util/watch-error-and-throw.util';

definePageMeta({ requireLoggedin: true });

const route = useRoute();
const code = route.params.code as string;

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);
const userService = new UserService($localApi);

const { theme } = await useTheme(null);
const { user } = await useUser(userService);
const { group, members, days, before, groupInfoError } = useGroup(code, groupService);
watchErrorAndThrow(groupInfoError);

const isOwned = computed(() =>
    members.value?.some(m => m.owner && m.id === user.value?.id) ?? false
);

const move = (forward: boolean) => {
  before.value = addDays(before.value, forward ? -7 : 7);
};

async function kick(index: number) {
  const member = members.value!.splice(index, 1)[0]!;

  try {
    await groupService.kickMember(code, member.id);
  } catch (e) {
    members.value!.splice(index, 0, member);
    throw e;
  }
}

async function leave() {
  await groupService.leaveGroup(code);
  await navigateTo('/groups/');
}
</script>