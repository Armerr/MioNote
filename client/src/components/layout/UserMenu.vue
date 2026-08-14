<template>
  <DropdownMenuRoot>
    <DropdownMenuTrigger as-child>
      <slot name="trigger" />
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
      <DropdownMenuContent
        align="end"
        :side-offset="10"
        class="z-50 w-[min(20rem,calc(100vw-1rem))] origin-top rounded-xl border border-theme-border bg-theme-canvas p-2 text-theme-text shadow-xl outline-none data-[state=closed]:animate-[mionote-pop-out_110ms_ease-in] data-[state=open]:animate-[mionote-pop-in_150ms_cubic-bezier(0.16,1,0.3,1)] motion-reduce:animate-none dark:bg-theme-background-elevated"
      >
        <div class="px-3 py-2.5">
          <p class="truncate text-sm font-semibold text-theme-text">
            {{ username }}
          </p>
          <p class="mt-0.5 text-xs text-theme-text-very-muted">MioNote</p>
        </div>

        <DropdownMenuSeparator class="my-1 h-px bg-theme-border" />

        <div
          class="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-3 rounded-lg px-3 py-2.5"
        >
          <span class="text-sm font-medium text-theme-text-muted">{{
            t("nav.language")
          }}</span>
          <LanguageToggle />
        </div>

        <div
          class="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-3 rounded-lg px-3 py-2.5"
        >
          <span
            class="flex min-w-0 items-center gap-2.5 text-sm font-medium text-theme-text-muted"
          >
            <FilePenLine class="h-4 w-4" />
            {{ t("nav.defaultEditor") }}
          </span>
          <SegmentedControl
            :model-value="defaultEditorMode"
            :items="editorModeItems"
            :aria-label="t('nav.defaultEditor')"
            @update:model-value="setDefaultEditorMode"
          />
        </div>

        <DropdownMenuItem
          class="flex cursor-pointer items-center justify-between rounded-md px-3 py-2.5 text-sm text-theme-text-muted outline-none transition-colors data-[highlighted]:bg-theme-background-elevated data-[highlighted]:text-theme-text"
          @select.prevent="changeTheme"
        >
          <span class="flex items-center gap-2.5">
            <Moon class="h-4 w-4" />
            {{ t("nav.darkMode") }}
          </span>
          <span
            class="relative h-5 w-9 rounded-full bg-theme-background-elevated transition-colors"
            :class="{ 'bg-theme-brand': darkTheme }"
            aria-hidden="true"
          >
            <span
              class="absolute left-0.5 top-0.5 h-4 w-4 rounded-full bg-theme-canvas shadow-sm transition-transform"
              :class="{ 'translate-x-4': darkTheme }"
            />
          </span>
        </DropdownMenuItem>

        <DropdownMenuSeparator class="my-1 h-px bg-theme-border" />

        <DropdownMenuItem
          class="flex cursor-pointer items-center gap-2.5 rounded-md px-3 py-2.5 text-sm text-theme-danger outline-none transition-colors data-[highlighted]:bg-theme-danger/10"
          @select="emit('sign-out')"
        >
          <LogOut class="h-4 w-4" />
          {{ t("nav.signOut") }}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenuPortal>
  </DropdownMenuRoot>
</template>

<script setup lang="ts">
import { FilePenLine, LogOut, Moon } from "lucide-vue-next";
import {
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuRoot,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "reka-ui";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import LanguageToggle from "../common/LanguageToggle.vue";
import SegmentedControl from "../ui/SegmentedControl.vue";
import { toggleTheme } from "../../utils/helpers";

defineProps({
  username: { type: String, default: "MioNote" },
});

const emit = defineEmits(["sign-out"]);
const { t } = useI18n();
const darkTheme = ref(document.body.classList.contains("dark"));
const defaultEditorMode = ref(
  localStorage.getItem("defaultEditorMode") === "markdown"
    ? "markdown"
    : "wysiwyg",
);
const editorModeItems = computed(() => [
  { value: "wysiwyg", label: t("editor.richText") },
  { value: "markdown", label: t("editor.markdown") },
]);

function changeTheme() {
  toggleTheme();
  darkTheme.value = document.body.classList.contains("dark");
}

function setDefaultEditorMode(mode) {
  defaultEditorMode.value = mode;
  localStorage.setItem("defaultEditorMode", mode);
}
</script>
