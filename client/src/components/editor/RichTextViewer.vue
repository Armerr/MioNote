<template>
  <EditorContent :editor="editorContent" class="mionote-rich-text viewer" />
</template>

<script setup lang="ts">
import { Editor, EditorContent } from "@tiptap/vue-3";
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";

import { createNoteExtensions } from "./extensions";
import "./richTextContent.css";

const props = defineProps({
  html: { type: String, default: "" },
});

const editor = ref<Editor | null>(null);
const editorContent = computed(() => editor.value as unknown as Editor);

onMounted(() => {
  editor.value = new Editor({
    extensions: createNoteExtensions({ editor: false }),
    content: props.html || "",
    editable: false,
    autofocus: false,
  });
});

watch(
  () => props.html,
  (value) => {
    editor.value?.commands.setContent(value || "", { emitUpdate: false });
  },
);

onBeforeUnmount(() => {
  editor.value?.destroy();
  editor.value = null;
});
</script>
