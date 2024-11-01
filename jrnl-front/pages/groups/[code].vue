<template>
  <div>
    <div v-if="group">
      <h1>{{ group.name }}</h1>
      <h2>{{ isOwned ? 'owned' : 'not owned' }}</h2>
    </div>
    <div v-else>
      <h1>loading group info...</h1>
    </div>

    <div v-if="members">
      <h2>Members</h2>
      <ul>
        <li v-for="(member, index) in members" :key="member.id">
          <p>{{ member.name }} // {{ member.id }}</p>
          <div v-if="isOwned && member.id !== supabaseUser?.id" @click="kick(index)">kick</div>
        </li>
      </ul>
    </div>
    <div v-else>
      <h2>loading members...</h2>
    </div>
  </div>
</template>

<script setup lang="ts">
import { GroupService } from '~/services/group.service';
import { useGroup } from '~/composables/group.composable';

const route = useRoute();
const { code } = route.params;

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);

const supabaseUser = useSupabaseUser();
let { group, members, days, before, execute } = useGroup(code as string, groupService);

onMounted(execute);

const isOwned = computed(() => members.value?.some(m => m.owner && m.id === supabaseUser.value?.id));

const dateWindowRange = computed(() => {
  // window size is 7 by default
  let start = before.value;
  if (!start) {
    const f = days.value?.[0]?.date;
    start = f ? new Date(f) : new Date();
  }

  const end = new Date(start);
  end.setDate(end.getDate() + 7);

  return { start, end };
});

const dateWindow = computed(() => {
  const { start, end } = dateWindowRange.value;
  return days.value?.filter(w => {
    const d = new Date(w.date);
    return d >= start && d < end;
  });
});

const move = (days: number) => {
  if (days > 0 && before.value === null) {
    // have already reached the newest
    return false;
  }

  const { start } = dateWindowRange.value;

  const d = new Date(start);

  d.setDate(d.getDate() + days);

  before.value = d;
};

async function kick(index: number) {
  const member = members.value!.splice(index, 1)[0]!;

  try {
    await groupService.kickMember(code as string, member.id);
  } catch (e) {
    console.error(e);
    members.value!.splice(index, 0, member);
  }
}

async function leave() {
  try {
    await groupService.leaveGroup(code as string);
    await navigateTo('/groups/');
  } catch (e) {
    console.error(e);
    // todo error alert
  }
}
</script>