<template>
  <div class="p-4 rounded-lg bg-colors-primary-900/40 space-y-4">
    <h2 class="text-lg font-light text-colors-primary-200">create a group</h2>

    <form class="space-y-3" @submit.prevent="createGroup">
      <input
          v-model="groupName"
          type="text"
          placeholder="group name"
          class="w-full px-3 py-2 rounded-md bg-colors-primary-800/60 text-colors-primary-100 placeholder:text-colors-primary-600 border border-colors-primary-700 focus:border-colors-primary-500 outline-none"
      >

      <button
          type="submit"
          :disabled="!groupName.length"
          class="w-full px-4 py-2 rounded-md bg-colors-primary-700 hover:bg-colors-primary-600 disabled:opacity-50 disabled:cursor-not-allowed text-colors-primary-100 transition-colors"
      >
        create
      </button>
    </form>
  </div>
</template>

<script lang="ts" setup>
import { GroupService } from '~/services/group.service';

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);

const groupName = ref('');
const emit = defineEmits<{
  'group-created': []
}>();

async function createGroup() {
  if (!groupName.value.length) {
    return;
  }

  const group = await groupService.createGroup(groupName.value);
  groupName.value = '';
  emit('group-created');

  await navigateTo({ name: 'groups-code', params: { code: group.code } });
}
</script>