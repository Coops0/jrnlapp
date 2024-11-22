<template>
  <div>
    <div class="fixed inset-0 bg-colors-primary-950/90 backdrop-blur-sm flex items-center justify-center p-4 z-30">
      <div class="bg-colors-primary-900 rounded-xl w-full max-w-md shadow-xl border border-colors-primary-800">
        <div v-if="showFullEntry" class="p-6 space-y-4">
          <!-- eslint-disable vue/no-v-html -->
          <pre
              v-if="showFullEntry.text?.length"
              class="text-colors-primary-200 whitespace-pre-wrap break-words text-sm"
              v-html="showFullEntry.text"
          />

          <div class="flex justify-end">
            <FormButton
                variant="primary"
                class="lowercase"
                @click="() => showFull(null)"
            >
              close
            </FormButton>
          </div>
        </div>
        <div v-else class="p-6 space-y-4">
          <div class="space-y-2">
            <h2 class="text-xl font-semibold text-colors-primary-100 lowercase">save conflict</h2>
            <p class="text-sm text-colors-primary-400 lowercase">your saved entry differs from the current local one</p>
          </div>

          <div class="space-y-4">
            <div class="space-y-2">
              <div class="relative">
                <div
                    class="absolute top-0 left-0 right-0 h-8 bg-gradient-to-b from-colors-primary-900 to-transparent z-10"/>
                <div
                    class="p-4 bg-colors-primary-800/50 rounded-lg min-h-[80px] max-h-[200px] overflow-y-auto transition-all hover:shadow-lg hover:bg-colors-primary-800/60"
                    @click="() => showFull(local)">
                  <!-- eslint-disable vue/no-v-html -->
                  <pre
                      v-if="localText?.length"
                      class="text-colors-primary-200 lowercase whitespace-pre-wrap break-words text-sm"
                      v-html="localText"
                  />
                  <p v-else class="text-colors-primary-400 lowercase italic">empty entry</p>
                  <span class="text-xs text-colors-primary-400 mt-2 block lowercase">local version</span>
                </div>
              </div>
            </div>

            <div class="space-y-2">
              <div class="relative">
                <div
                    class="absolute top-0 left-0 right-0 h-8 bg-gradient-to-b from-colors-primary-900 to-transparent z-10"/>
                <div
                    class="p-4 bg-colors-primary-800/50 rounded-lg min-h-[80px] max-h-[200px] overflow-y-auto transition-all hover:shadow-lg hover:bg-colors-primary-800/60"
                    @click="() => showFull(server)">
                  <!-- eslint-disable vue/no-v-html -->
                  <pre
                      v-if="serverText?.length"
                      class="text-colors-primary-200 lowercase whitespace-pre-wrap break-words text-sm"
                      v-html="serverText"
                  />
                  <p v-else class="text-colors-primary-400 lowercase italic">empty entry</p>
                  <span class="text-xs text-colors-primary-400 mt-2 block lowercase">server version</span>
                </div>
              </div>
            </div>
          </div>

          <div class="flex flex-col sm:flex-row gap-3 pt-2">
            <FormButton
                variant="danger"
                class="flex-1 lowercase"
                @click="emit('resolve', false)"
            >
              use local version
            </FormButton>
            <FormButton
                variant="primary"
                class="flex-1 lowercase"
                @click="emit('resolve', true)"
            >
              use server version
            </FormButton>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Entry } from '~/types/entry.type';

const props = defineProps<{
  local: Entry;
  server: Entry;
}>();

const emit = defineEmits<{ resolve: [server: boolean] }>();

const lastBitOfText = (text: string) => {
  if (!text) return '';

  const lines = text.split(/\n|\r\n/);
  if (lines.length <= 10) {
    return text;
  }

  return `...\n${lines.slice(-10).join('\n')}`;
};

const localText = computed(() => props.local.text ? lastBitOfText(props.local.text.trim()) : '');
const serverText = computed(() => props.server.text ? lastBitOfText(props.server.text.trim()) : '');

const showFullEntry = ref<Entry | null>(null);

function showFull(entry: Entry | null) {
  showFullEntry.value = entry;
}
</script>