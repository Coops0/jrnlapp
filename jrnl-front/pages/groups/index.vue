<template>
  <div>
    <h1>Groups</h1>
    <div>
      <div v-for="group in groups" :key="group.code">
        <NuxtLink :to="{ name: 'groups-code', params: { code: group.code } }">
          <div>{{ group.name }}</div>
        </NuxtLink>
      </div>

      <div>
        <input v-model="groupName" placeholder="Group name"/>
        <button @click="createGroup">Create Group</button>
      </div>

      <div>
        <input v-model="joinGroupCode" placeholder="Group code" @input="updateGroupSearchResults"/>

        <div v-if="groupSearchResults">
          <div>{{ groupSearchResults.name }}</div>
          <div>{{ groupSearchResults.members }} members</div>
          <button @click="joinGroup">Join Group</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { GroupService } from '~/services/group.service';
import type { Group } from '~/types/group.type';

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);

const cachedGroups = useLocalStorage('groups', [] as Group[]);
const { data: groups, refresh } = useLazyAsyncData(
    'groups',
    () => groupService.getJoinedGroups(),
    {
      default() {
        return cachedGroups.value;
      }
    }
);

onMounted(refresh);

watchImmediate(groups, (g) => {
  if (g) {
    cachedGroups.value = g;
  }
}, { deep: true });

const groupName = ref('');
const joinGroupCode = ref('');
const groupSearchResults = ref<{ id: string; name: string; members: number; } | null>(null);

const updateGroupSearchResults = useDebounceFn(async () => {
  const c = joinGroupCode.value;
  groupSearchResults.value = c.length ? await groupService.getGroup(joinGroupCode.value) : null;
}, 250);

async function joinGroup() {
  const c = joinGroupCode.value;
  if (!c.length) {
    return;
  }

  joinGroupCode.value = '';

  await groupService.joinGroup(c);
  await refresh();
}

async function createGroup() {
  const n = groupName.value;
  if (!n.length) {
    return;
  }

  groupName.value = '';
  const group = await groupService.createGroup(n);
  // todo proper sorting
  cachedGroups.value.push(group);

  await navigateTo({ name: 'groups-code', params: { code: group.code } });
}

</script>