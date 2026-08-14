<template>
  <input
    ref="input"
    v-bind="inputAttrs"
    v-model="model"
    :class="inputClasses"
    :type="type"
  />
</template>

<script setup lang="ts">
import { computed, ref, useAttrs } from "vue";

import { cn } from "../../utils/cn";

defineOptions({ inheritAttrs: false });

const model = defineModel<string | number>({ default: "" });
defineProps({
  type: { type: String, default: "text" },
});

const attrs = useAttrs();
const input = ref();
const inputClasses = computed(() =>
  cn(
    "flex h-11 w-full rounded-md border border-theme-border bg-theme-canvas px-3 py-2 text-base text-theme-text outline-none transition-colors placeholder:text-theme-text-very-muted focus:border-theme-brand focus:ring-2 focus:ring-theme-brand/20 disabled:cursor-not-allowed disabled:opacity-50 sm:h-10 sm:text-sm dark:bg-theme-background-elevated",
    attrs.class,
  ),
);
const inputAttrs = computed(() => {
  const { class: _class, ...rest } = attrs;
  return rest;
});

defineExpose({
  focus: () => input.value?.focus(),
  get selectionStart() {
    return input.value?.selectionStart || 0;
  },
});
</script>
