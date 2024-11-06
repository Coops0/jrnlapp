<template>
  <editor-content :editor="editor"/>
</template>

<script lang="ts" setup>
import { EditorContent, useEditor } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';

const props = defineProps<{ modelValue?: string }>();
const emit = defineEmits<{
  'update:modelValue': [content: string]
}>();

watch(() => props.modelValue, value => {
  const e = editor.value;
  if (e && e.getHTML() !== value) {
    editor.value?.commands.setContent(value ?? '', false);
  }
});

const editor = useEditor({
  content: props.modelValue ?? '',
  extensions: [StarterKit],
  editorProps: {
    attributes: {
      class: 'prose prose-sm sm:prose-base lg:prose-lg xl:prose-2xl m-5 focus:outline-none max-w-none prose-invert h-full',
    }
  },
  autofocus: true,
  onUpdate({ editor: e }) {
    const t = e.getHTML();
    if (t.length > 7500) {
      alert('too long... reduce to avoid loss of data');
    }
    emit('update:modelValue', e.getHTML());
  },
});
</script>