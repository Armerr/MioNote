<template>
  <DialogRoot v-model:open="open">
    <DialogPortal>
      <DialogOverlay
        class="fixed inset-0 z-50 bg-black/35 backdrop-blur-[2px] data-[state=closed]:animate-[mionote-overlay-out_120ms_ease-in] data-[state=open]:animate-[mionote-overlay-in_150ms_ease-out] motion-reduce:animate-none"
        :class="lightweight ? 'max-sm:backdrop-blur-none' : ''"
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
const props = defineProps({ lightweight: Boolean });
const attrs = useAttrs();
const contentClasses = computed(() =>
  cn(
    "fixed left-1/2 top-1/2 z-50 w-[calc(100%-1rem)] max-w-lg -translate-x-1/2 -translate-y-1/2 rounded-xl border border-theme-border bg-theme-canvas p-4 text-theme-text shadow-xl outline-none motion-reduce:animate-none sm:w-[calc(100%-2rem)] sm:rounded-lg sm:p-5 dark:bg-theme-background-elevated",
    props.lightweight
      ? "data-[state=open]:animate-[mionote-overlay-in_120ms_ease-out] data-[state=closed]:animate-[mionote-overlay-out_100ms_ease-in]"
      : "data-[state=open]:animate-[mionote-modal-in_180ms_cubic-bezier(0.16,1,0.3,1)] data-[state=closed]:animate-[mionote-modal-out_120ms_ease-in]",
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
