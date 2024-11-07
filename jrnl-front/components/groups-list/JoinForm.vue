<template>
  <div class="p-4 rounded-lg bg-colors-primary-900/40 space-y-4">
    <h2 class="text-lg font-light text-colors-primary-200">find your group</h2>

    <div class="space-y-3">
      <input
          v-model="joinGroupCode"
          class="w-full px-3 py-2 rounded-md bg-colors-primary-800/60 text-colors-primary-100 placeholder:text-colors-primary-600 border border-colors-primary-700 focus:border-colors-primary-500 outline-none"
          placeholder="group code"
          type="text"
          @input="updateGroupSearchResults"
      >

      <div v-if="status === 'pending'">
        <p class="text-colors-primary-400">loading...</p>
      </div>
      <div
          v-else-if="groupSearchResults"
          class="p-3 rounded-md bg-colors-primary-800/60 space-y-2"
      >
        <div class="flex items-center justify-between">
          <span class="text-colors-primary-200">{{ groupSearchResults.name }}</span>
          <span class="text-sm text-colors-primary-400">
            {{ groupSearchResults.members }} members
          </span>
        </div>

        <button
            class="w-full px-4 py-2 rounded-md bg-colors-primary-700 hover:bg-colors-primary-600 text-colors-primary-100 transition-colors"
            @click="joinGroup"
        >
          join
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { GroupService } from '~/services/group.service';

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);

const joinGroupCode = ref('');

const emit = defineEmits<{
  'group-joined': []
}>();

const { data: groupSearchResults, refresh, status, clear } = useAsyncData(
    'group-search-results',
    () => groupService.getGroup(joinGroupCode.value),
    {
      immediate: false,
      lazy: true
    }
);

const updateGroupSearchResults = useDebounceFn(async () => {
  if (joinGroupCode.value) {
    await refresh();
  } else {
    clear();
  }
}, 250);

async function joinGroup() {
  const code = joinGroupCode.value;
  if (!code.length) {
    clear();
    return;
  }

  await groupService.joinGroup(code);
  joinGroupCode.value = '';
  emit('group-joined');
}
</script>