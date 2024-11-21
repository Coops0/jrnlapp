<template>
  <div class="p-4 rounded-lg bg-colors-primary-900/40 space-y-4">
    <h2 class="text-lg font-light text-colors-primary-200">create a group</h2>

    <form class="space-y-3" @submit.prevent="createGroup">
      <FormTextInput
          v-model.trim="groupName"
          placeholder="group name"
      />

      <FormButton
          :disabled="!groupName.length || isCreatingLoading"
          full
          size="md"
          variant="primary"
          @click="createGroup"
      >
        create
      </FormButton>
    </form>
  </div>
</template>

<script lang="ts" setup>
import { GroupService } from '../../services/group.service';

const { $localApi } = useNuxtApp();
const groupService = new GroupService($localApi);

const groupName = ref('');
const emit = defineEmits<{
  'group-created': []
}>();

const isCreatingLoading = ref(false);

async function createGroup() {
  if (!groupName.value.length) {
    return;
  }

  isCreatingLoading.value = true;
  let group;
  try {
    group = await groupService.createGroup(groupName.value);
  } finally {
    isCreatingLoading.value = false;
  }
  groupName.value = '';
  emit('group-created');

  await navigateTo({ name: 'groups-code', params: { code: group.code } });
}
</script>