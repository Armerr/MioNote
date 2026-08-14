<template>
  <nav
    class="fixed inset-x-0 bottom-0 z-40 flex h-[calc(64px+env(safe-area-inset-bottom))] items-center justify-between border-t border-theme-border bg-theme-header/95 px-3 pb-[env(safe-area-inset-bottom)] shadow-[0_-8px_24px_rgb(38_38_35/0.06)] backdrop-blur lg:hidden"
  >
    <Button
      variant="ghost"
      size="sm"
      class="h-12 min-w-[64px] flex-col gap-0.5 px-2 text-[11px]"
      @click="emit('open-notes')"
    >
      <NotebookText class="h-5 w-5" />
      {{ t("sidebar.notes") }}
    </Button>
    <Button
      variant="ghost"
      size="sm"
      class="h-12 min-w-[64px] flex-col gap-0.5 px-2 text-[11px]"
      @click="emit('open-search')"
    >
      <Search class="h-5 w-5" />
      {{ t("nav.search") }}
    </Button>
    <Button
      size="sm"
      class="h-11 min-w-[86px] gap-1.5 rounded-full px-3 shadow-md"
      @click="router.push({ name: 'new' })"
    >
      <Plus class="h-4 w-4" />
      {{ t("sidebar.newNote") }}
    </Button>
  </nav>
</template>

<script setup lang="ts">
import { NotebookText, Plus, Search } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import Button from "../ui/Button.vue";

const emit = defineEmits(["open-notes", "open-search"]);
const { t } = useI18n();
const router = useRouter();
</script>
