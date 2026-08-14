<template>
  <Teleport to="body">
    <Transition name="mionote-panel">
      <div v-if="open" class="fixed inset-0 z-[70]" @contextmenu.prevent>
        <button
          v-if="isMobile"
          class="absolute inset-0 cursor-default bg-black/20 backdrop-blur-[1px]"
          :aria-label="t('common.close')"
          @click="close"
        />
        <section
          ref="menuElement"
          class="note-context-menu fixed overflow-visible rounded-xl border border-theme-border bg-theme-canvas text-theme-text shadow-2xl dark:bg-theme-background-elevated"
          :class="[
            isHorizontal
              ? 'flex max-w-[calc(100vw-1rem)] items-center gap-1 overflow-x-auto whitespace-nowrap p-1.5'
              : compact
                ? 'w-[min(12rem,calc(100vw-1rem))] p-1'
                : 'w-[min(17rem,calc(100vw-1rem))] p-1.5',
            isMobile
              ? 'inset-x-2 bottom-2 w-auto rounded-2xl p-2 pb-[calc(0.5rem+env(safe-area-inset-bottom))]'
              : '',
          ]"
          :style="isMobile ? undefined : menuStyle"
          @click.stop
          @contextmenu.prevent
        >
          <template
            v-for="(item, index) in items"
            :key="`${item.id || 'separator'}-${index}`"
          >
            <div
              v-if="item.separator"
              class="my-1.5 h-px bg-theme-border"
            ></div>
            <div
              v-else
              class="relative"
              :class="isHorizontal ? 'shrink-0' : ''"
              @mouseenter="
                !isMobile && item.children && (activeSubmenu = item.id)
              "
            >
              <button
                type="button"
                class="flex min-h-11 items-center gap-3 rounded-lg px-3 text-left text-[15px] font-medium outline-none transition-[background-color,color,transform] duration-150 hover:bg-theme-background-elevated active:scale-[0.985] disabled:pointer-events-none disabled:opacity-45"
                :class="[
                  item.danger
                    ? 'text-theme-danger hover:bg-theme-danger/10'
                    : 'text-theme-text',
                  isHorizontal
                    ? 'w-auto min-w-11 justify-center gap-2 px-3'
                    : 'w-full',
                ]"
                :disabled="item.disabled"
                @click="selectItem(item)"
              >
                <component
                  :is="item.icon"
                  v-if="item.icon"
                  class="h-[18px] w-[18px] shrink-0 text-theme-text-muted"
                  :class="item.danger ? 'text-theme-danger' : ''"
                />
                <span
                  class="min-w-0 truncate"
                  :class="isHorizontal ? '' : 'flex-1'"
                  >{{ item.label }}</span
                >
                <component
                  :is="item.trailingIcon"
                  v-if="item.trailingIcon"
                  class="h-4 w-4 shrink-0 text-theme-text-very-muted"
                />
                <ChevronRight
                  v-else-if="item.children"
                  class="h-4 w-4 shrink-0 text-theme-text-very-muted"
                />
              </button>

              <div
                v-if="item.children && activeSubmenu === item.id"
                class="border-theme-border bg-theme-canvas text-theme-text shadow-xl dark:bg-theme-background-elevated"
                :class="
                  isMobile
                    ? 'mx-1 mb-1 rounded-lg border p-1'
                    : 'absolute left-[calc(100%+0.5rem)] top-0 w-44 rounded-xl border p-1.5'
                "
              >
                <button
                  v-for="child in item.children"
                  :key="child.id"
                  type="button"
                  class="flex min-h-10 w-full items-center gap-2 rounded-md px-2.5 text-left text-sm text-theme-text-muted outline-none transition-colors hover:bg-theme-background-elevated hover:text-theme-text"
                  :class="
                    child.active ? 'bg-theme-brand-soft text-theme-text' : ''
                  "
                  @click="selectItem(child)"
                >
                  <Check
                    v-if="child.active"
                    class="h-4 w-4 shrink-0 text-theme-brand-strong"
                  />
                  <span :class="child.active ? '' : 'pl-6'">{{
                    child.label
                  }}</span>
                </button>
              </div>
            </div>
          </template>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { Check, ChevronRight } from "lucide-vue-next";
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import type { MenuItem } from "../../types";

const props = withDefaults(
  defineProps<{
    items?: MenuItem[];
    layout?: "vertical" | "horizontal";
    compact?: boolean;
    position?: { x: number; y: number };
  }>(),
  {
    items: () => [],
    layout: "vertical",
    compact: false,
    position: () => ({ x: 16, y: 16 }),
  },
);
const emit = defineEmits<{ (event: "select", id: string): void }>();
const open = defineModel({ type: Boolean, default: false });
const { t } = useI18n();
const activeSubmenu = ref<string | null>(null);
const isMobile = ref(false);
const menuElement = ref<HTMLElement | null>(null);
let mediaQuery: MediaQueryList | undefined;
const isHorizontal = computed(() => props.layout === "horizontal");

const menuStyle = computed(() => {
  const width = isHorizontal.value ? 430 : props.compact ? 192 : 272;
  const height = isHorizontal.value ? 64 : 360;
  const x = Math.max(
    8,
    Math.min(props.position.x || 16, window.innerWidth - width - 8),
  );
  const y = Math.max(
    8,
    Math.min(props.position.y || 16, window.innerHeight - height - 8),
  );
  return { left: `${x}px`, top: `${y}px` };
});

function updateMobileMode() {
  isMobile.value = mediaQuery?.matches || false;
}

function selectItem(item: MenuItem) {
  if (item.disabled) return;
  if (item.children) {
    activeSubmenu.value = activeSubmenu.value === item.id ? null : item.id;
    return;
  }
  emit("select", item.id ?? "");
  close();
}

function close() {
  open.value = false;
  activeSubmenu.value = null;
}

function handlePointerDown(event: PointerEvent) {
  if (event.target instanceof Node && menuElement.value?.contains(event.target))
    return;
  close();
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") close();
}

watch(open, (visible) => {
  if (!visible) activeSubmenu.value = null;
});

onMounted(() => {
  mediaQuery = window.matchMedia("(max-width: 639px)");
  updateMobileMode();
  mediaQuery.addEventListener("change", updateMobileMode);
  document.addEventListener("pointerdown", handlePointerDown);
  document.addEventListener("keydown", handleKeydown);
});

onBeforeUnmount(() => {
  mediaQuery?.removeEventListener("change", updateMobileMode);
  document.removeEventListener("pointerdown", handlePointerDown);
  document.removeEventListener("keydown", handleKeydown);
});
</script>
