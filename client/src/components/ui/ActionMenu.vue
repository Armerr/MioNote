<template>
  <DropdownMenuRoot>
    <DropdownMenuTrigger as-child>
      <slot name="trigger" />
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
      <DropdownMenuContent
        :align="align"
        class="z-50 min-w-48 origin-top rounded-md border border-theme-border bg-theme-canvas p-1 text-theme-text shadow-lg outline-none data-[state=closed]:animate-[mionote-pop-out_100ms_ease-in] data-[state=open]:animate-[mionote-pop-in_140ms_cubic-bezier(0.16,1,0.3,1)] motion-reduce:animate-none dark:bg-theme-background-elevated"
        :side-offset="8"
      >
        <template
          v-for="(item, index) in items"
          :key="`${item.label || 'separator'}-${index}`"
        >
          <DropdownMenuSeparator
            v-if="item.separator"
            class="my-1 h-px bg-theme-border"
          />
          <DropdownMenuItem
            v-else
            :disabled="item.disabled"
            class="flex cursor-pointer items-center gap-2 rounded px-2.5 py-2 text-sm text-theme-text-muted outline-none transition-colors data-[disabled]:pointer-events-none data-[highlighted]:bg-theme-background-elevated data-[highlighted]:text-theme-text data-[disabled]:opacity-50"
            @select="run(item)"
          >
            <component
              :is="item.icon"
              v-if="item.icon"
              class="h-4 w-4 shrink-0"
            />
            <span class="flex-1">{{ item.label }}</span>
            <kbd
              v-if="item.keyboardShortcut"
              class="rounded border border-theme-border bg-theme-background px-1.5 py-0.5 text-[10px] text-theme-text-very-muted"
              >{{ item.keyboardShortcut }}</kbd
            >
          </DropdownMenuItem>
        </template>
      </DropdownMenuContent>
    </DropdownMenuPortal>
  </DropdownMenuRoot>
</template>

<script setup lang="ts">
import {
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuRoot,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "reka-ui";

import type { MenuAlignment, MenuItem } from "../../types";

withDefaults(
  defineProps<{
    items?: MenuItem[];
    align?: MenuAlignment;
  }>(),
  {
    items: () => [],
    align: "end",
  },
);

function run(item: MenuItem) {
  item.command?.();
}
</script>
