<template>
  <Sheet
    v-model="open"
    class="inset-0 w-full border-0 bg-theme-canvas shadow-none lg:hidden"
  >
    <header
      class="flex h-16 shrink-0 items-center justify-between border-b border-theme-border px-5"
    >
      <div
        class="flex items-center gap-2.5 text-base font-semibold text-theme-text"
      >
        <span
          class="grid h-8 w-8 place-items-center rounded-md bg-theme-brand-soft text-theme-brand-strong"
        >
          <NotebookText class="h-4 w-4" />
        </span>
        {{ t("sidebar.allNotes") }}
      </div>
      <Button
        variant="ghost"
        size="icon-sm"
        :aria-label="t('common.close')"
        @click="open = false"
      >
        <X class="h-5 w-5" />
      </Button>
    </header>

    <div class="flex shrink-0 items-center justify-end px-5 pb-1 pt-3">
      <Button size="sm" @click="createNote">
        <Plus class="h-4 w-4" />{{ t("sidebar.newNote") }}
      </Button>
    </div>
    <NoteList grouped @navigate="open = false" />
  </Sheet>
</template>

<script setup lang="ts">
import { NotebookText, Plus, X } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import Button from "../ui/Button.vue";
import Sheet from "../ui/Sheet.vue";
import NoteList from "../notes/NoteList.vue";

const open = defineModel({ type: Boolean, default: false });
const { t } = useI18n();
const router = useRouter();

function createNote() {
  open.value = false;
  router.push({ name: "new" });
}
</script>
