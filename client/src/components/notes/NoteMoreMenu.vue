<template>
  <DropdownMenuRoot>
    <DropdownMenuTrigger as-child>
      <slot name="trigger" />
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
      <DropdownMenuContent
        align="end"
        :side-offset="10"
        class="z-50 w-[min(14rem,calc(100vw-1rem))] origin-top rounded-xl border border-theme-border bg-theme-canvas p-1.5 text-theme-text shadow-2xl outline-none data-[state=closed]:animate-[mionote-pop-out_110ms_ease-in] data-[state=open]:animate-[mionote-pop-in_160ms_cubic-bezier(0.16,1,0.3,1)] motion-reduce:animate-none sm:w-[min(18rem,calc(100vw-1rem))] sm:p-2 dark:bg-theme-background-elevated"
      >
        <template
          v-for="(item, index) in items"
          :key="`${item.label || 'separator'}-${index}`"
        >
          <DropdownMenuSeparator
            v-if="item.separator"
            class="my-1.5 h-px bg-theme-border sm:my-2"
            :class="item.mobileOnly ? 'sm:hidden' : ''"
          />
          <DropdownMenuSub v-else-if="item.children?.length">
            <DropdownMenuSubTrigger
              :disabled="item.disabled"
              class="flex min-h-10 cursor-pointer items-center gap-2.5 rounded-md px-2.5 text-sm text-theme-text outline-none transition-colors data-[disabled]:pointer-events-none data-[highlighted]:bg-theme-background-elevated data-[disabled]:opacity-45 sm:min-h-12 sm:gap-3 sm:rounded-lg sm:px-3 sm:text-[17px]"
              :class="item.mobileOnly ? 'sm:hidden' : ''"
            >
              <component
                :is="item.icon"
                v-if="item.icon"
                class="h-[1.125rem] w-[1.125rem] shrink-0 sm:h-5 sm:w-5"
              />
              <span class="flex-1">{{ item.label }}</span>
              <ChevronRight
                class="h-[1.125rem] w-[1.125rem] text-theme-text-very-muted sm:h-5 sm:w-5"
              />
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent
                class="z-50 min-w-40 origin-left rounded-xl border border-theme-border bg-theme-canvas p-1.5 text-theme-text shadow-2xl outline-none data-[state=closed]:animate-[mionote-pop-out_100ms_ease-in] data-[state=open]:animate-[mionote-pop-in_140ms_cubic-bezier(0.16,1,0.3,1)] sm:min-w-48 sm:p-2 dark:bg-theme-background-elevated"
                :side-offset="8"
              >
                <DropdownMenuItem
                  v-for="child in item.children"
                  :key="child.label"
                  :disabled="child.disabled"
                  class="flex min-h-10 cursor-pointer items-center gap-2.5 rounded-md px-2.5 text-sm text-theme-text outline-none transition-colors data-[disabled]:pointer-events-none data-[highlighted]:bg-theme-background-elevated data-[disabled]:opacity-45 sm:min-h-11 sm:gap-3 sm:rounded-lg sm:px-3 sm:text-[17px]"
                  @select="run(child)"
                >
                  <component
                    :is="child.icon"
                    v-if="child.icon"
                    class="h-[1.125rem] w-[1.125rem] shrink-0 sm:h-5 sm:w-5"
                  />
                  <span>{{ child.label }}</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>
          <DropdownMenuItem
            v-else
            :disabled="item.disabled"
            class="flex min-h-10 cursor-pointer items-center gap-2.5 rounded-md px-2.5 text-sm text-theme-text outline-none transition-colors data-[disabled]:pointer-events-none data-[highlighted]:bg-theme-background-elevated data-[disabled]:opacity-45 sm:min-h-12 sm:gap-3 sm:rounded-lg sm:px-3 sm:text-[17px]"
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
              class="h-[1.125rem] w-[1.125rem] shrink-0 sm:h-5 sm:w-5"
            />
            <span class="flex-1">{{ item.label }}</span>
            <ChevronRight
              v-if="item.chevron"
              class="h-[1.125rem] w-[1.125rem] text-theme-text-very-muted sm:h-5 sm:w-5"
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
