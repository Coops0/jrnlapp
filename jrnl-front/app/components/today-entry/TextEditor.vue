<template>
  <div>
    <EditorContent v-if="editor" :editor class="h-full"/>
    <!-- eslint-disable vue/no-v-html -->
    <div v-else :class="editorClasses" v-html="modelValue?.length ? modelValue : initial"/>
  </div>
</template>

<script lang="ts" setup>
import { EditorContent, useEditor } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';

const props = defineProps<{
  modelValue?: string;
  initial?: string | null;
}>();

const emit = defineEmits<{
  'update:modelValue': [content: string]
}>();

const editorClasses = 'prose prose-sm sm:prose-base lg:prose-lg xl:prose-2xl prose-p:text-colors-primary-200 prose-strong:text-colors-primary-300 prose-headings:text-colors-primary-100 m-5 !outline-0 max-w-full h-full better-outline-none';

watch(() => props.modelValue, value => {
  const e = editor.value;
  if (e && e.getHTML() !== value) {
    editor.value?.commands.setContent(value ?? '', false);
  }
});

const editor = useEditor({
  content: (props.modelValue?.length ? props.modelValue : props.initial) ?? '',
  extensions: [StarterKit],
  editorProps: {
    attributes: {
      class: editorClasses,
    }
  },
  autofocus: true,
  onUpdate({ editor: e }) {
    const t = e.getHTML();
    if (t.length > 10000) {
      alert('too long... reduce to avoid loss of data');
    }
    emit('update:modelValue', e.getHTML());
  },
});

function handleGlobalKeydown(event: KeyboardEvent) {
  if (editor.value?.isFocused) {
    return;
  }

  if (event.metaKey || event.ctrlKey || event.altKey || event.key.length > 1) {
    return;
  }

  event.preventDefault();

  editor.value?.commands?.focus?.();
  editor.value?.options?.element?.dispatchEvent(new KeyboardEvent('keydown', {
    key: event.key,
    code: event.code,
    ctrlKey: event.ctrlKey,
    metaKey: event.metaKey,
    altKey: event.altKey,
    shiftKey: event.shiftKey,
  }));
}

onMounted(() => {
  document.addEventListener('keydown', handleGlobalKeydown);
});

onUnmounted(() => {
  try {
    editor.value?.destroy();
  } catch {
    /* empty */
  }

  document.removeEventListener('keydown', handleGlobalKeydown);
});
</script>

<style>
/* tailwind adds other properties */
/*noinspection CssUnusedSymbol*/
.better-outline-none {
  outline: none;
}
</style>