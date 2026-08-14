<template>
  <ToggleGroupRoot
    type="single"
    :model-value="modelValue"
    :disabled="disabled"
    :aria-label="ariaLabel"
    class="inline-flex h-8 shrink-0 items-center gap-0.5 rounded-md border border-theme-border bg-theme-background-elevated p-0.5 text-xs font-medium shadow-sm max-sm:rounded-full"
    @update:model-value="handleUpdate"
  >
    <ToggleGroupItem
      v-for="item in items"
      :key="item.value"
      :value="item.value"
      :disabled="item.disabled"
      class="inline-flex h-7 items-center gap-1 whitespace-nowrap rounded px-2 text-theme-text-muted outline-none transition-[background-color,color,box-shadow,transform] duration-150 hover:text-theme-text focus-visible:ring-2 focus-visible:ring-theme-brand/60 active:scale-[0.97] disabled:pointer-events-none disabled:opacity-45 data-[state=on]:bg-theme-canvas data-[state=on]:text-theme-text data-[state=on]:shadow-sm max-sm:data-[state=on]:bg-theme-brand/10 max-sm:data-[state=on]:text-theme-brand-strong max-sm:data-[state=on]:shadow-none"
      :class="
        compactOnMobile ? 'max-sm:w-7 max-sm:justify-center max-sm:px-0' : ''
      "
      :title="item.title || item.label"
      :aria-label="item.ariaLabel || item.label"
    >
      <component
        v-if="item.icon"
        :is="item.icon"
        class="h-3.5 w-3.5 max-sm:h-4 max-sm:w-4"
      />
      <span :class="compactOnMobile ? 'max-sm:sr-only' : ''">{{
        item.label
      }}</span>
    </ToggleGroupItem>
  </ToggleGroupRoot>
</template>

<script setup lang="ts">
import { ToggleGroupItem, ToggleGroupRoot } from "reka-ui";

import type { SegmentedControlItem } from "../../types";

withDefaults(
  defineProps<{
    modelValue?: string;
    items?: SegmentedControlItem[];
    ariaLabel?: string;
    disabled?: boolean;
    compactOnMobile?: boolean;
  }>(),
  {
    modelValue: "",
    items: () => [],
    ariaLabel: "",
    disabled: false,
    compactOnMobile: false,
  },
);

const emit = defineEmits<{
  (event: "update:modelValue", value: string): void;
}>();

function handleUpdate(value: string | undefined) {
  if (value) emit("update:modelValue", value);
}
</script>
