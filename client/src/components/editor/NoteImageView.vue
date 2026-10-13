<template>
  <NodeViewWrapper as="div" class="mionote-image">
    <img
      :src="displaySrc"
      :alt="node.attrs.alt || ''"
      :title="node.attrs.title || undefined"
    />
  </NodeViewWrapper>
</template>

<script setup lang="ts">
import { NodeViewWrapper, nodeViewProps } from "@tiptap/vue-3";
import { ref, watch } from "vue";

import { isAttachmentSrc, resolveAttachmentUrl } from "../../utils/attachmentUrls";

const props = defineProps(nodeViewProps);

const displaySrc = ref("");

function resolve(src: unknown) {
  const value = typeof src === "string" ? src : "";
  if (isAttachmentSrc(value)) {
    resolveAttachmentUrl(value)
      .then((url) => {
        displaySrc.value = url;
      })
      .catch(() => {
        // Leave the original path so the broken-image state is visible.
        displaySrc.value = value;
      });
  } else {
    displaySrc.value = value;
  }
}

watch(() => props.node.attrs.src, resolve, { immediate: true });
</script>

<style scoped>
.mionote-image img {
  cursor: default;
}
</style>
