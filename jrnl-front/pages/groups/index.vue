<template>
  <div>
    <h1>Groups</h1>
  </div>
</template>

<script setup lang="ts">
import { GroupService } from '~/services/group.service';

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);

const { data: groups, error, status } = useAsyncData('groups', () => groupService.getJoinedGroups());

async function getGroup(code: string): ReturnType<typeof groupService.getGroup> {
  return await groupService.getGroup(code);
}

async function joinGroup(code: string): ReturnType<typeof groupService.joinGroup> {
  return await groupService.joinGroup(code);
}

async function createGroup(): ReturnType<typeof groupService.createGroup> {
  return await groupService.createGroup(name);
}


</script>