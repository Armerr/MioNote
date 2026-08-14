<template>
  <Sheet v-model="open" class="lg:hidden">
    <header
      class="flex items-center justify-between border-b border-theme-border px-5 py-4"
    >
      <div class="flex items-center gap-2 text-sm font-semibold">
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
        <X class="h-4 w-4" />
      </Button>
    </header>

    <div class="flex items-center justify-between px-5 pb-2 pt-5">
      <p class="text-[11px] font-semibold uppercase text-theme-text-very-muted">
        {{ t("sidebar.recent") }}
      </p>
      <Button size="sm" @click="createNote"
        ><Plus class="h-4 w-4" />{{ t("sidebar.newNote") }}</Button
      >
    </div>
    <NoteList @navigate="open = false" />
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
