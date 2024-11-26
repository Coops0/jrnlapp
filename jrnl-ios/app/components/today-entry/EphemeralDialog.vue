<template>
  <FormDialog
      v-model="toggled"
      show-close
      v-slot="{ close }"
  >
    <div class="space-y-4">
      <div class="text-xl font-medium text-colors-primary-100 mb-2">enable ephemeral mode?</div>
      <div class="text-colors-primary-300 space-y-2">
        <p>this entry will automatically be deleted</p>
        <div class="bg-colors-primary-700/50 p-3 rounded-lg text-colors-accent-400 font-medium">
          <NuxtTime :datetime="tomorrow" relative/>
        </div>
        <p class="text-sm">
          ephemeral mode means this entry will disappear after the day ends
        </p>
      </div>

      <div class="flex justify-end gap-3 pt-2">
        <button
            class="px-4 py-2 rounded-lg text-colors-primary-300 hover:text-colors-primary-100 transition-colors"
            @click="close"
        >
          cancel
        </button>
        <button
            class="px-4 py-2 rounded-lg bg-colors-accent-400/20 text-colors-accent-400 hover:bg-colors-accent-400/30 transition-colors"
            @click="() => emit('confirm')"
        >
          enable
        </button>
      </div>
    </div>
  </FormDialog>
</template>

<script lang="ts" setup>
const toggled = defineModel<boolean>({ required: true, default: false });
defineProps<{ tomorrow: Date }>();

const emit = defineEmits<{ confirm: [] }>();
</script>