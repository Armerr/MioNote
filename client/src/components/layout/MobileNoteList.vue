<template>
  <div
    class="flex min-h-0 flex-1 flex-col bg-theme-canvas pt-[max(0.75rem,env(safe-area-inset-top))]"
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
      <div class="flex items-center gap-1">
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
        <Button
          v-if="showClose"
          variant="ghost"
          size="icon-sm"
          :aria-label="t('common.close')"
          @click="emit('close')"
        >
          <X class="h-5 w-5" />
        </Button>
      </div>
    </header>

    <div class="flex items-center gap-2 px-5 pb-2 pt-3">
      <div
        class="min-w-0 flex-1 rounded-lg bg-theme-background-elevated px-1 ring-1 ring-theme-border/70"
      >
        <SearchInput
          ref="searchInput"
          :placeholder="t('search.placeholder')"
          @search="emit('navigate')"
        />
      </div>
      <Button size="sm" @click="createNote">
        <Plus class="h-4 w-4" />{{ t("sidebar.newNote") }}
      </Button>
    </div>
    <NoteList grouped @navigate="emit('navigate')" />
  </div>
</template>

<script setup lang="ts">
import { NotebookText, Plus, X } from "lucide-vue-next";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { clearStoredToken } from "../../api/tokenStorage";
import { resetAuthCheck } from "../../router";
import { useGlobalStore } from "../../stores/globalStore";
import NoteList from "../notes/NoteList.vue";
import SearchInput from "../search/SearchInput.vue";
import Button from "../ui/Button.vue";
import UserMenu from "./UserMenu.vue";

withDefaults(
  defineProps<{
    showClose?: boolean;
  }>(),
  {
    showClose: false,
  },
);
const emit = defineEmits(["close", "navigate"]);
const { t } = useI18n();
const globalStore = useGlobalStore();
const router = useRouter();
const searchInput = ref();
const userInitial = computed(
  () => globalStore.currentUser?.username?.charAt(0).toUpperCase() || "M",
);

function createNote() {
  emit("navigate");
  router.push({ name: "new" });
}

function logOut() {
  clearStoredToken();
  globalStore.currentUser = null;
  resetAuthCheck();
  emit("close");
  router.push({ name: "login" });
}

defineExpose({
  focusSearch: () => searchInput.value?.focus(),
});
</script>
