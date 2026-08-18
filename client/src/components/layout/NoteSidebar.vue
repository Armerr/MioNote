<template>
  <aside
    class="hidden h-full w-full shrink-0 border-r border-theme-border bg-theme-canvas md:flex md:flex-col"
  >
    <div class="flex items-center justify-between px-5 pb-2 pt-5">
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
        <UserMenu
          :username="globalStore.currentUser?.username"
          @sign-out="logOut"
        >
          <template #trigger>
            <Button
              variant="ghost"
              size="icon-sm"
              class="rounded-full p-0.5"
              :title="t('nav.userMenu')"
              :aria-label="t('nav.userMenu')"
            >
              <span
                class="grid h-7 w-7 place-items-center rounded-full bg-theme-brand-soft text-xs font-semibold text-theme-brand-strong"
              >
                {{ userInitial }}
              </span>
            </Button>
          </template>
        </UserMenu>
      </div>
    </div>

    <div class="flex items-center gap-2 px-5 pb-3">
      <div
        class="min-w-0 flex-1 rounded-lg bg-theme-background-elevated px-1 ring-1 ring-theme-border/70"
      >
        <SearchInput ref="searchInput" :placeholder="t('search.placeholder')" />
      </div>
      <Button
        size="icon-sm"
        :title="t('nav.newNote')"
        :aria-label="t('nav.newNote')"
        @click="router.push({ name: 'new' })"
      >
        <Plus class="h-4 w-4" />
      </Button>
    </div>

    <NoteList />
  </aside>
</template>

<script setup lang="ts">
import { ArrowDownAZ, ListFilter, NotebookText, Plus } from "lucide-vue-next";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { clearStoredToken } from "../../api/tokenStorage";
import { resetAuthCheck } from "../../router";
import { useGlobalStore } from "../../stores/globalStore";
import { params, searchSortOptions } from "../../utils/constants";
import NoteList from "../notes/NoteList.vue";
import SearchInput from "../search/SearchInput.vue";
import ActionMenu from "../ui/ActionMenu.vue";
import Button from "../ui/Button.vue";
import UserMenu from "./UserMenu.vue";

const { t } = useI18n();
const globalStore = useGlobalStore();
const router = useRouter();
const searchInput = ref();
const userInitial = computed(
  () => globalStore.currentUser?.username?.charAt(0).toUpperCase() || "M",
);
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

function showAll(sortBy: number) {
  router.push({
    name: "search",
    query: { [params.searchTerm]: "*", [params.sortBy]: sortBy },
  });
}

function logOut() {
  clearStoredToken();
  globalStore.currentUser = null;
  resetAuthCheck();
  router.push({ name: "login" });
}

defineExpose({
  focusSearch: () => searchInput.value?.focus(),
});
</script>
