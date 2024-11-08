<template>
  <section v-if="members" class="space-y-4">
    <div class="flex items-center justify-between">
      <h2 class="text-lg md:text-xl font-light text-colors-primary-200">people</h2>
      <span class="text-sm md:text-base text-colors-primary-400">{{ members.length }} people</span>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
      <div
          v-for="(member, index) in members"
          :key="member.id"
          :class="{
              'border-2 border-colors-accent-400': member.id === id,
              'border-2 border-colors-primary-700': member.owner,
            }"
          class="flex items-center justify-between p-3 md:p-4 rounded-lg bg-colors-primary-900/40"
      >
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 md:w-10 md:h-10 rounded-full bg-colors-primary-700 flex items-center justify-center">
              <span class="text-sm md:text-base text-colors-primary-200">
                {{ member.name[0]?.toUpperCase() }}
              </span>
          </div>
          <span class="text-colors-primary-200 md:text-lg">{{ member.name.toLowerCase() }}</span>
        </div>

        <FormButton
            v-if="isOwned && member.id !== id"
            size="sm"
            variant="danger"
            @click="emit('kick', index)"
        >x
        </FormButton>
      </div>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type { FetchedGroupMember } from '~/services/group.service';

defineProps<{
  members: FetchedGroupMember[];
  id: string;
  isOwned: boolean;
}>();

const emit = defineEmits<{
  kick: [index: number]
}>();
</script>

