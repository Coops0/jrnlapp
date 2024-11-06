<template>
  <div class="max-w-2xl mx-auto px-4 space-y-8">
    <section v-if="groups?.length" class="grid gap-3">
      <GroupsListCard
          v-for="group in groups"
          :key="group.code"
          :name="group.name"
          :code="group.code"
      />
    </section>

    <section v-else class="py-8 text-center bg-colors-primary-900/20 rounded-lg">
      <p class="text-colors-primary-400">you haven't joined any groups</p>
    </section>

    <div class="grid gap-4 sm:grid-cols-2">
      <GroupsListCreateForm @group-created="refresh"/>
      <GroupsListJoinForm @group-joined="refresh"/>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { GroupService } from '~/services/group.service';

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);

const { data: groups, refresh } = useLazyAsyncData('groups', () => groupService.getJoinedGroups());
</script>