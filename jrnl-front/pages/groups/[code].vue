<template>
  <div>
    <div v-if="group">
      <h1>{{ group.name }}</h1>
      <h2>{{ group.isOwned ? 'owned' : 'not owned' }}</h2>
    </div>
    <div v-else>
      <h1>loading group info...</h1>
    </div>

    <div v-if="members">
      <h2>Members</h2>
      <ul>
        <li v-for="(member, index) in members" :key="member.id">
          <p>{{ member.name }} // {{ member.id }}</p>
          <div v-if="group?.isOwned" @click="kick(index)">kick</div>
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

let { group, members, weekly } = useGroup(code as string, groupService, supabaseUser);


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