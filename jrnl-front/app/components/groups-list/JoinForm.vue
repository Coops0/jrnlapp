<template>
  <div class="p-4 rounded-lg bg-colors-primary-900/40 space-y-4">
    <h2 class="text-lg font-light text-colors-primary-200">find your group</h2>

    <div class="space-y-3">
      <FormTextInput
          v-model="joinGroupCode"
          placeholder="group code"
          @input="updateGroupSearchResults"
      />
      <div
          v-if="groupSearchResults"
          class="p-3 rounded-md bg-colors-primary-800/60 space-y-2"
      >
        <div class="flex items-center justify-between">
          <span class="text-colors-primary-200">{{ groupSearchResults.name }}</span>
          <span class="text-sm text-colors-primary-400">
            {{ groupSearchResults.members }} members
          </span>
        </div>

        <FormButton
            full
            size="md"
            variant="primary"
            @click="joinGroup"
        >
          join
        </FormButton>
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

const { data: groupSearchResults, refresh, clear } = useAsyncData(
    'group-search-results',
    () => groupService.getGroup(joinGroupCode.value),
    {
      immediate: false,
      lazy: true
    }
);

async function updateGroupSearchResults() {
  if (joinGroupCode.value) {
    await refresh();
  } else {
    clear();
  }
}

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