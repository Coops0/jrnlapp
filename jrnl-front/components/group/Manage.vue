<template>
  <div class="space-y-2">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl md:text-3xl font-light text-colors-primary-100">{{ group.name }}</h1>
        <p class="text-sm md:text-base text-colors-primary-400 font-bold">
          {{ isOwned ? 'you own this group' : '' }}
        </p>
      </div>

      <FormButton
          variant="danger"
          size="sm"
          @click="emit('leave')"
      >
        leave
      </FormButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { GroupInfo } from '~/composables/group.composable';

defineProps<{
  group: GroupInfo;
  isOwned: boolean;
}>();

const emit = defineEmits<{ leave: [] }>();
</script>

