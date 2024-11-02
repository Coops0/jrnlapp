<template>
  <editor-content :editor="editor"/>
</template>

<script setup lang="ts">
import { EditorContent, useEditor } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';

const props = defineProps<{ modelValue?: string }>();
const emit = defineEmits<{
  'update:modelValue': [content: string]
}>();

watch(() => props.modelValue, value => {
  const e = editor.value;
  if (!e) return;

  if (e.getHTML() !== value) {
    editor.value?.commands.setContent(value ?? '', false);
  }
});

const editor = useEditor({
  content: props.modelValue ?? '',
  extensions: [StarterKit],
  editorProps: {
    attributes: {
      class: 'm-5 focus:outline-none font-code'
    }
  },
  autofocus: true,
  onUpdate({ editor: e }) {
    emit('update:modelValue', e.getHTML());
  },
});


</script>

<style scoped>
/**
.tiptap {
  :first-child {
    margin-top: 0;
  }

  ul,
  ol {
    padding: 0 1rem;
    margin: 1.25rem 1rem 1.25rem 0.4rem;

    li p {
      margin-top: 0.25em;
      margin-bottom: 0.25em;
    }
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    line-height: 1.1;
    margin-top: 2.5rem;
    text-wrap: pretty;
  }

  h1,
  h2 {
    margin-top: 3.5rem;
    margin-bottom: 1.5rem;
  }

  h1 {
    font-size: 1.4rem;
  }

  h2 {
    font-size: 1.2rem;
  }

  h3 {
    font-size: 1.1rem;
  }

  h4,
  h5,
  h6 {
    font-size: 1rem;
  }

  code {
    background-color: mediumpurple;
    border-radius: 0.4rem;
    color: black;
    font-size: 0.85rem;
    padding: 0.25em 0.3em;
  }

  pre {
    background: black;
    border-radius: 0.5rem;
    color: white;
    font-family: 'JetBrainsMono', monospace;
    margin: 1.5rem 0;
    padding: 0.75rem 1rem;

    code {
      background: none;
      color: inherit;
      font-size: 0.8rem;
      padding: 0;
    }
  }

  blockquote {
    border-left: 3px solid lightgray;
    margin: 1.5rem 0;
    padding-left: 1rem;
  }

  hr {
    border: none;
    border-top: 1px solid lightgray;
    margin: 2rem 0;
  }
}
**/
</style>