<template>
  <aside
    class="hidden h-full w-full shrink-0 border-r border-theme-border bg-theme-canvas lg:flex lg:flex-col"
  >
    <div class="flex items-center justify-between px-5 pb-3 pt-5">
      <div
        class="flex items-center gap-3 text-lg font-semibold text-theme-text"
      >
        <span
          class="grid h-9 w-9 place-items-center rounded-md bg-theme-background-elevated text-theme-text-muted"
        >
          <NotebookText class="h-5 w-5" />
        </span>
        {{ t("sidebar.allNotes") }}
      </div>
      <div class="flex items-center gap-1">
        <Button
          variant="ghost"
          size="icon"
          :title="t('nav.newNote')"
          :aria-label="t('nav.newNote')"
          @click="router.push({ name: 'new' })"
        >
          <Plus class="h-6 w-6" />
        </Button>
        <ActionMenu :items="menuItems" align="end">
          <template #trigger>
            <Button
              variant="ghost"
              size="icon-sm"
              :aria-label="
                t('search.sortBy', { name: t('search.lastModified') })
              "
            >
              <ListFilter class="h-4 w-4" />
            </Button>
          </template>
        </ActionMenu>
        <Button
          variant="ghost"
          size="icon-sm"
          :title="t('nav.search')"
          :aria-label="t('nav.search')"
          @click="emit('open-search')"
        >
          <Search class="h-4 w-4" />
        </Button>
      </div>
    </div>

    <NoteList />
  </aside>
</template>

<script setup lang="ts">
import {
  ArrowDownAZ,
  ListFilter,
  NotebookText,
  Plus,
  Search,
} from "lucide-vue-next";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import ActionMenu from "../ui/ActionMenu.vue";
import Button from "../ui/Button.vue";
import { params, searchSortOptions } from "../../utils/constants";
import NoteList from "../notes/NoteList.vue";

const { t } = useI18n();
const emit = defineEmits(["open-search"]);
const router = useRouter();
const menuItems = computed(() => [
  {
    label: t("search.sortBy", { name: t("search.lastModified") }),
    icon: ListFilter,
    command: () => showAll(searchSortOptions.lastModified),
  },
  {
    label: t("search.sortBy", { name: t("search.title") }),
    icon: ArrowDownAZ,
    command: () => showAll(searchSortOptions.title),
  },
]);

function showAll(sortBy) {
  router.push({
    name: "search",
    query: { [params.searchTerm]: "*", [params.sortBy]: sortBy },
  });
}
</script>
