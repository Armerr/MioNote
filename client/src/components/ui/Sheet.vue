<template>
  <DialogRoot v-model:open="open">
    <DialogPortal>
      <DialogOverlay
        class="fixed inset-0 z-50 bg-black/35 backdrop-blur-[2px] data-[state=closed]:animate-[mionote-overlay-out_120ms_ease-in] data-[state=open]:animate-[mionote-overlay-in_150ms_ease-out] motion-reduce:animate-none"
      />
      <DialogContent
        v-bind="contentAttrs"
        :class="contentClasses"
        @escape-key-down="close"
        @pointer-down-outside="close"
      >
        <slot />
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>

<script setup lang="ts">
import {
  DialogContent,
  DialogOverlay,
  DialogPortal,
  DialogRoot,
} from "reka-ui";
import { computed, useAttrs } from "vue";

import { cn } from "../../utils/cn";

defineOptions({ inheritAttrs: false });

const open = defineModel({ type: Boolean, default: false });
const attrs = useAttrs();
const contentClasses = computed(() =>
  cn(
    "fixed inset-y-0 left-0 z-50 flex w-[min(22rem,calc(100%-1rem))] flex-col border-r border-theme-border bg-theme-sidebar text-theme-text shadow-2xl outline-none data-[state=open]:animate-[mionote-sheet-in_180ms_cubic-bezier(0.16,1,0.3,1)] data-[state=closed]:animate-[mionote-sheet-out_120ms_ease-in] motion-reduce:animate-none",
    attrs.class,
  ),
);
const contentAttrs = computed(() => {
  const { class: _class, ...rest } = attrs;
  return rest;
});

function close() {
  open.value = false;
}
</script>
