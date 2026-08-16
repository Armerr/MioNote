<template>
  <div
    class="editor-toolbar flex shrink-0 border-b border-theme-border bg-theme-canvas py-1 pl-[var(--note-page-margin,1.5rem)] pr-1.5 sm:h-12 sm:items-center sm:gap-0.5 sm:overflow-x-auto sm:py-0 sm:pr-3"
    :class="props.previewing ? 'sm:hidden' : ''"
  >
    <div class="flex min-w-0 flex-1 items-center gap-0.5 sm:hidden">
      <slot name="mobile-controls" />
    </div>
    <div
      v-if="!props.previewing"
      class="mobile-format-toolbar fixed inset-x-0 bottom-0 z-40 grid h-[calc(3.25rem+env(safe-area-inset-bottom))] grid-cols-8 items-start border-t border-theme-border bg-theme-canvas/95 px-1 pt-1 shadow-[0_-8px_24px_rgb(38_38_35/0.06)] backdrop-blur sm:contents"
    >
      <ActionMenu :items="headingItems" align="start">
        <template #trigger>
          <Button
            variant="ghost"
            size="icon-sm"
            :title="t('editor.heading')"
            :aria-label="t('editor.heading')"
            class="gap-0.5 px-1.5 sm:w-auto"
          >
            <Heading1 class="h-4 w-4" />
            <ChevronDown class="h-3.5 w-3.5" />
          </Button>
        </template>
      </ActionMenu>
      <ActionMenu :items="fontSizeItems" align="start">
        <template #trigger>
          <Button
            variant="ghost"
            size="icon-sm"
            :title="t('editor.font')"
            :aria-label="t('editor.font')"
            class="gap-0.5 px-1.5 sm:w-auto"
          >
            <span class="text-sm leading-none">16</span>
            <ChevronDown class="h-3.5 w-3.5" />
          </Button>
        </template>
      </ActionMenu>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.bold')"
        tooltip-shortcut="⌘ B"
        :aria-label="t('editor.bold')"
        @click="run('bold')"
      >
        <Bold class="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.italic')"
        tooltip-shortcut="⌘ I"
        :aria-label="t('editor.italic')"
        @click="run('italic')"
      >
        <Italic class="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.underline')"
        :aria-label="t('editor.underline')"
        @click="applyInline('u')"
      >
        <Underline class="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.strike')"
        :aria-label="t('editor.strike')"
        @click="run('strike')"
      >
        <Strikethrough class="h-4 w-4" />
      </Button>
      <PopoverRoot v-model:open="highlightPaletteOpen">
        <PopoverTrigger as-child>
          <Button
            variant="ghost"
            size="icon-sm"
            :title="t('editor.highlight')"
            :aria-label="t('editor.highlight')"
          >
            <Highlighter class="h-4 w-4 text-amber-500" />
          </Button>
        </PopoverTrigger>
        <PopoverPortal>
          <PopoverContent
            class="color-palette"
            align="start"
            :side="colorPaletteSide"
            :side-offset="8"
          >
            <div class="grid grid-cols-5 gap-1.5">
              <button
                v-for="color in highlightSwatches"
                :key="color"
                type="button"
                class="color-swatch"
                :style="{ backgroundColor: color }"
                :aria-label="color"
                @click="applyHighlight(color)"
              ></button>
              <label
                class="color-swatch color-swatch-custom"
                :aria-label="t('editor.customColor')"
              >
                <input
                  v-model="customHighlightColor"
                  type="color"
                  @change="applyHighlight(customHighlightColor)"
                />
              </label>
            </div>
          </PopoverContent>
        </PopoverPortal>
      </PopoverRoot>
      <PopoverRoot v-model:open="colorPaletteOpen">
        <PopoverTrigger as-child>
          <Button
            variant="ghost"
            size="icon-sm"
            :title="t('editor.fontColor')"
            :aria-label="t('editor.fontColor')"
            @pointerdown.capture="saveTextColorSelection"
          >
            <Palette class="h-4 w-4" :style="{ color: selectedTextColor }" />
            <ChevronDown class="-ml-1 h-3 w-3" />
          </Button>
        </PopoverTrigger>
        <PopoverPortal>
          <PopoverContent
            class="color-palette"
            align="start"
            :side="colorPaletteSide"
            :side-offset="8"
          >
            <div class="grid grid-cols-5 gap-1.5">
              <button
                v-for="color in textColorSwatches"
                :key="color"
                type="button"
                class="color-swatch"
                :style="{ backgroundColor: color }"
                :aria-label="color"
                @pointerdown.prevent
                @click="applyTextColor(color)"
              ></button>
              <label
                class="color-swatch color-swatch-custom"
                :aria-label="t('editor.customColor')"
              >
                <input
                  v-model="customTextColor"
                  type="color"
                  @change="applyTextColor(customTextColor)"
                />
              </label>
            </div>
          </PopoverContent>
        </PopoverPortal>
      </PopoverRoot>
    </div>

    <div
      v-if="!props.previewing"
      class="hidden sm:mx-1 sm:block sm:h-5 sm:w-px sm:shrink-0 sm:bg-theme-border"
    ></div>

    <div
      v-if="!props.previewing"
      class="contents max-sm:hidden sm:flex sm:shrink-0 sm:items-center sm:gap-0.5"
    >
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.taskList')"
        :aria-label="t('editor.taskList')"
        @click="run('taskList')"
      >
        <ListTodo class="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.orderedList')"
        :aria-label="t('editor.orderedList')"
        @click="run('orderedList')"
      >
        <ListOrdered class="h-4 w-4" />
      </Button>
      <ActionMenu :items="alignmentItems" align="start">
        <template #trigger>
          <Button
            variant="ghost"
            size="icon-sm"
            :title="t('editor.alignment')"
            :aria-label="t('editor.alignment')"
          >
            <AlignLeft class="h-4 w-4" />
            <ChevronDown class="-ml-1 h-3 w-3" />
          </Button>
        </template>
      </ActionMenu>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="
          formatPainterArmed
            ? t('editor.applyFormatPainter')
            : t('editor.formatPainter')
        "
        :aria-label="
          formatPainterArmed
            ? t('editor.applyFormatPainter')
            : t('editor.formatPainter')
        "
        :class="
          formatPainterArmed
            ? 'bg-theme-background-elevated text-theme-brand-strong'
            : ''
        "
        @click="toggleFormatPainter"
      >
        <Paintbrush class="h-4 w-4" />
      </Button>
    </div>

    <div
      v-if="!props.previewing"
      class="hidden sm:mx-1 sm:block sm:h-5 sm:w-px sm:shrink-0 sm:bg-theme-border"
    ></div>

    <div
      v-if="!props.previewing"
      class="contents max-sm:hidden sm:flex sm:shrink-0 sm:items-center sm:gap-0.5"
    >
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.media')"
        :aria-label="t('editor.media')"
        @click="chooseMedia"
      >
        <ImagePlus class="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.table')"
        :aria-label="t('editor.table')"
        @click="tableDialogOpen = true"
      >
        <Table2 class="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('editor.attachment')"
        :aria-label="t('editor.attachment')"
        @click="chooseAttachment"
      >
        <Paperclip class="h-4 w-4" />
      </Button>
    </div>
  </div>

  <input
    ref="mediaInput"
    class="hidden"
    type="file"
    accept="image/*,video/*"
    @change="uploadMedia"
  />
  <input
    ref="attachmentInput"
    class="hidden"
    type="file"
    @change="uploadAttachment"
  />

  <Dialog v-model="tableDialogOpen" :aria-label="t('editor.table')">
    <div class="space-y-4">
      <div>
        <h2 class="text-base font-semibold">{{ t("editor.insertTable") }}</h2>
        <p class="mt-1 text-sm text-theme-text-muted">
          {{ t("editor.insertTableHint") }}
        </p>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <label class="space-y-1.5 text-sm font-medium">
          <span>{{ t("editor.tableRows") }}</span>
          <Input v-model.number="tableRows" type="number" min="2" max="20" />
        </label>
        <label class="space-y-1.5 text-sm font-medium">
          <span>{{ t("editor.tableColumns") }}</span>
          <Input v-model.number="tableColumns" type="number" min="1" max="12" />
        </label>
      </div>
      <div class="flex justify-end gap-2 pt-1">
        <Button variant="ghost" @click="tableDialogOpen = false">{{
          t("common.cancel")
        }}</Button>
        <Button @click="insertTable">{{ t("common.confirm") }}</Button>
      </div>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import {
  AlignCenter,
  AlignJustify,
  AlignLeft,
  AlignRight,
  Bold,
  ChevronDown,
  Heading1,
  Highlighter,
  ImagePlus,
  Italic,
  ListOrdered,
  ListTodo,
  Paintbrush,
  Palette,
  Paperclip,
  Strikethrough,
  Table2,
  Underline,
} from "lucide-vue-next";
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import {
  PopoverContent,
  PopoverPortal,
  PopoverRoot,
  PopoverTrigger,
} from "reka-ui";

import ActionMenu from "../ui/ActionMenu.vue";
import Button from "../ui/Button.vue";
import Dialog from "../ui/Dialog.vue";
import Input from "../ui/Input.vue";
import type { EditorSelectionStyle, ToastEditorHandle } from "../../types";

const props = withDefaults(
  defineProps<{
    editor?: ToastEditorHandle | null;
    previewing?: boolean;
  }>(),
  {
    editor: null,
    previewing: false,
  },
);

const { t } = useI18n();
const mediaInput = ref<HTMLInputElement | null>(null);
const attachmentInput = ref<HTMLInputElement | null>(null);
const tableDialogOpen = ref(false);
const tableRows = ref(2);
const tableColumns = ref(2);
const formatPainterArmed = ref(false);
const copiedTextStyle = ref<EditorSelectionStyle | null>(null);
const highlightPaletteOpen = ref(false);
const colorPaletteOpen = ref(false);
const customHighlightColor = ref("#fef08a");
const customTextColor = ref("#2563eb");
const selectedTextColor = ref("#111827");
const textColorSelection = ref<[number, number] | null>(null);
const isCompactViewport = ref(false);
const colorPaletteSide = computed(() =>
  isCompactViewport.value ? "top" : "bottom",
);

const highlightSwatches = [
  "#fef3c7",
  "#fde68a",
  "#fed7aa",
  "#fecaca",
  "#fbcfe8",
  "#e9d5ff",
  "#c7d2fe",
  "#bfdbfe",
  "#bae6fd",
  "#a7f3d0",
  "#bbf7d0",
  "#d9f99d",
  "#fef08a",
  "#e5e7eb",
  "#ffffff",
];

const textColorSwatches = [
  "#111827",
  "#4b5563",
  "#9ca3af",
  "#dc2626",
  "#ea580c",
  "#ca8a04",
  "#16a34a",
  "#0d9488",
  "#0284c7",
  "#2563eb",
  "#4f46e5",
  "#7c3aed",
  "#c026d3",
  "#db2777",
  "#ffffff",
];

const headingItems = computed(() => [
  { label: t("editor.paragraph"), command: () => run("paragraph") },
  ...[1, 2, 3, 4, 5, 6].map((level) => ({
    label: t("editor.headingLevel", { level }),
    command: () => run("heading", { level }),
  })),
]);

const fontSizeItems = computed(() =>
  [12, 14, 16, 18, 20, 24, 28, 32].map((size) => ({
    label: t("editor.fontSize", { size }),
    command: () => applyInline("span", { "font-size": `${size}px` }),
  })),
);

const alignmentItems = computed(() => [
  {
    label: t("editor.alignLeft"),
    icon: AlignLeft,
    command: () => props.editor?.applyAlignment("left"),
  },
  {
    label: t("editor.alignCenter"),
    icon: AlignCenter,
    command: () => props.editor?.applyAlignment("center"),
  },
  {
    label: t("editor.alignRight"),
    icon: AlignRight,
    command: () => props.editor?.applyAlignment("right"),
  },
  {
    label: t("editor.alignJustify"),
    icon: AlignJustify,
    command: () => props.editor?.applyAlignment("justify"),
  },
]);

function syncCompactViewport() {
  isCompactViewport.value = window.matchMedia("(max-width: 639px)").matches;
}

function run(command: string, payload?: unknown) {
  props.editor?.exec(command, payload);
}

function applyInline(tag: string, style: Record<string, string> = {}) {
  return props.editor?.applyInlineStyle(tag, style) || false;
}

function applyHighlight(color: string) {
  applyInline("mark", { "background-color": color });
  highlightPaletteOpen.value = false;
}

function applyTextColor(color: string) {
  selectedTextColor.value = color;
  customTextColor.value = color;
  restoreTextColorSelection();
  applyInline("span", { color });
  colorPaletteOpen.value = false;
}

function saveTextColorSelection() {
  textColorSelection.value = props.editor?.getSelection() ?? null;
}

function restoreTextColorSelection() {
  if (!Array.isArray(textColorSelection.value)) return;
  props.editor?.setSelection?.(...textColorSelection.value);
}

function toggleFormatPainter() {
  if (formatPainterArmed.value && copiedTextStyle.value) {
    props.editor?.applyCopiedStyle?.(copiedTextStyle.value);
    formatPainterArmed.value = false;
    copiedTextStyle.value = null;
    return;
  }

  const selectionStyle = props.editor?.getSelectionStyle?.();
  if (!selectionStyle) return;
  copiedTextStyle.value = selectionStyle;
  formatPainterArmed.value = true;
}

function chooseMedia() {
  mediaInput.value?.click();
}

function uploadMedia(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (file) props.editor?.addMediaFile(file);
  input.value = "";
}

function chooseAttachment() {
  attachmentInput.value?.click();
}

function uploadAttachment(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (file) props.editor?.addAttachmentFile(file);
  input.value = "";
}

function insertTable() {
  run("addTable", {
    rowCount: Math.max(2, Math.min(20, Number(tableRows.value) || 2)),
    columnCount: Math.max(1, Math.min(12, Number(tableColumns.value) || 2)),
  });
  tableDialogOpen.value = false;
}

onMounted(() => {
  syncCompactViewport();
  window.addEventListener("resize", syncCompactViewport);
});

onBeforeUnmount(() => {
  window.removeEventListener("resize", syncCompactViewport);
});
</script>

<style scoped>
@media (max-width: 639px) {
  .editor-toolbar {
    gap: 0;
    padding-block: 0.125rem;
  }

  .editor-toolbar :deep(button:not([role="switch"])) {
    height: 2.25rem;
    min-height: 2.25rem;
  }

  .mobile-format-toolbar {
    padding-bottom: env(safe-area-inset-bottom);
  }

  .mobile-format-toolbar :deep([data-reka-dropdown-menu-trigger]),
  .mobile-format-toolbar :deep([data-slot="popover-trigger"]) {
    min-width: 0;
  }

  .mobile-format-toolbar :deep(button:not([role="switch"])) {
    height: 2.5rem;
    min-height: 2.5rem;
    width: 100%;
    min-width: 0;
    padding-inline: 0.125rem;
  }
}

@media (min-width: 640px) {
  .editor-toolbar {
    grid-template-rows: none;
    scrollbar-width: thin;
  }

  .editor-toolbar::-webkit-scrollbar {
    height: 3px;
  }

  .editor-toolbar::-webkit-scrollbar-thumb {
    background: rgb(var(--theme-border));
    border-radius: 999px;
  }
}

.color-palette {
  z-index: 60;
  width: 11.5rem;
  border: 1px solid rgb(var(--theme-border));
  border-radius: 0.625rem;
  background: rgb(var(--theme-canvas));
  padding: 0.625rem;
  box-shadow: 0 12px 32px rgb(15 23 42 / 16%);
  outline: none;
}

.color-swatch {
  height: 1.5rem;
  width: 1.5rem;
  cursor: pointer;
  border: 1px solid rgb(var(--theme-border));
  border-radius: 0.3rem;
  box-shadow: inset 0 0 0 1px rgb(255 255 255 / 20%);
  transition:
    transform 120ms ease,
    box-shadow 120ms ease;
}

.color-swatch:hover,
.color-swatch:focus-visible {
  transform: scale(1.12);
  box-shadow:
    0 0 0 2px rgb(var(--theme-canvas)),
    0 0 0 4px rgb(var(--theme-brand));
  outline: none;
}

.color-swatch-custom {
  position: relative;
  overflow: hidden;
  background: conic-gradient(
    #f43f5e,
    #f59e0b,
    #22c55e,
    #3b82f6,
    #a855f7,
    #f43f5e
  );
}

.color-swatch-custom input {
  position: absolute;
  inset: 0;
  height: 100%;
  width: 100%;
  cursor: pointer;
  opacity: 0;
}
</style>
