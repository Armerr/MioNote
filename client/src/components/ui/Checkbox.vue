<template>
  <CheckboxRoot
    v-model="model"
    :class="
      cn(
        'grid h-4 w-4 shrink-0 place-items-center rounded border border-theme-border bg-theme-canvas text-theme-brand outline-none transition-colors focus-visible:ring-2 focus-visible:ring-theme-brand/60 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-theme-brand data-[state=checked]:bg-theme-brand',
        $attrs.class,
      )
    "
  >
    <CheckboxIndicator
      class="grid place-items-center text-theme-brand-contrast"
    >
      <Check class="h-3.5 w-3.5" :stroke-width="3" />
    </CheckboxIndicator>
  </CheckboxRoot>
</template>

<script setup lang="ts">
import { Check } from "lucide-vue-next";
import { CheckboxIndicator, CheckboxRoot } from "reka-ui";

import { cn } from "../../utils/cn";

const model = defineModel({ type: Boolean, default: false });
</script>
