<template>
  <SwitchRoot
    v-model="model"
    :class="
      cn(
        'inline-flex h-6 w-10 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent bg-theme-background-elevated outline-none transition-colors focus-visible:ring-2 focus-visible:ring-theme-brand/60 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-theme-brand',
        $attrs.class,
      )
    "
  >
    <SwitchThumb
      class="pointer-events-none block h-5 w-5 rounded-full bg-theme-canvas shadow-sm transition-transform data-[state=checked]:translate-x-4"
    />
  </SwitchRoot>
</template>

<script setup lang="ts">
import { SwitchRoot, SwitchThumb } from "reka-ui";

import { cn } from "../../utils/cn";

const model = defineModel({ type: Boolean, default: false });
</script>
