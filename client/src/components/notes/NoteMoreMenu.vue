<template>
  <DropdownMenuRoot>
    <DropdownMenuTrigger as-child>
      <slot name="trigger" />
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
      <DropdownMenuContent
        align="end"
        :side-offset="10"
        class="z-50 w-[min(18rem,calc(100vw-1rem))] origin-top rounded-xl border border-theme-border bg-theme-canvas p-2 text-theme-text shadow-2xl outline-none data-[state=closed]:animate-[mionote-pop-out_110ms_ease-in] data-[state=open]:animate-[mionote-pop-in_160ms_cubic-bezier(0.16,1,0.3,1)] motion-reduce:animate-none dark:bg-theme-background-elevated"
      >
        <template
          v-for="(item, index) in items"
          :key="`${item.label || 'separator'}-${index}`"
        >
          <DropdownMenuSeparator
            v-if="item.separator"
            class="my-2 h-px bg-theme-border"
            :class="item.mobileOnly ? 'sm:hidden' : ''"
          />
          <DropdownMenuSub v-else-if="item.children?.length">
            <DropdownMenuSubTrigger
              :disabled="item.disabled"
              class="flex min-h-12 cursor-pointer items-center gap-3 rounded-lg px-3 text-base text-theme-text outline-none transition-colors data-[disabled]:pointer-events-none data-[highlighted]:bg-theme-background-elevated data-[disabled]:opacity-45 sm:text-[17px]"
              :class="item.mobileOnly ? 'sm:hidden' : ''"
            >
              <component
                :is="item.icon"
                v-if="item.icon"
                class="h-5 w-5 shrink-0"
              />
              <span class="flex-1">{{ item.label }}</span>
              <ChevronRight class="h-5 w-5 text-theme-text-very-muted" />
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent
                class="z-50 min-w-48 origin-left rounded-xl border border-theme-border bg-theme-canvas p-2 text-theme-text shadow-2xl outline-none data-[state=closed]:animate-[mionote-pop-out_100ms_ease-in] data-[state=open]:animate-[mionote-pop-in_140ms_cubic-bezier(0.16,1,0.3,1)] dark:bg-theme-background-elevated"
                :side-offset="8"
              >
                <DropdownMenuItem
                  v-for="child in item.children"
                  :key="child.label"
                  :disabled="child.disabled"
                  class="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg px-3 text-[17px] text-theme-text outline-none transition-colors data-[disabled]:pointer-events-none data-[highlighted]:bg-theme-background-elevated data-[disabled]:opacity-45"
                  @select="run(child)"
                >
                  <component
                    :is="child.icon"
                    v-if="child.icon"
                    class="h-5 w-5 shrink-0"
                  />
                  <span>{{ child.label }}</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>
          <DropdownMenuItem
            v-else
            :disabled="item.disabled"
            class="flex min-h-12 cursor-pointer items-center gap-3 rounded-lg px-3 text-base text-theme-text outline-none transition-colors data-[disabled]:pointer-events-none data-[highlighted]:bg-theme-background-elevated data-[disabled]:opacity-45 sm:text-[17px]"
            :class="[
              item.danger
                ? 'text-theme-danger data-[highlighted]:bg-theme-danger/10'
                : '',
              item.mobileOnly ? 'sm:hidden' : '',
            ]"
            @select="run(item)"
          >
            <component
              :is="item.icon"
              v-if="item.icon"
              class="h-5 w-5 shrink-0"
            />
            <span class="flex-1">{{ item.label }}</span>
            <ChevronRight
              v-if="item.chevron"
              class="h-5 w-5 text-theme-text-very-muted"
            />
          </DropdownMenuItem>
        </template>
      </DropdownMenuContent>
    </DropdownMenuPortal>
  </DropdownMenuRoot>
</template>

<script setup lang="ts">
import { ChevronRight } from "lucide-vue-next";
import {
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuRoot,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "reka-ui";

import type { MenuItem } from "../../types";

withDefaults(
  defineProps<{
    items?: MenuItem[];
  }>(),
  {
    items: () => [],
  },
);

function run(item: MenuItem) {
  item.command?.();
}
</script>
