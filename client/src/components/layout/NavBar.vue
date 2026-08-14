<template>
  <nav
    class="flex h-14 shrink-0 items-center border-b border-theme-border bg-theme-header px-3 sm:h-[70px] sm:px-6"
  >
    <div class="flex min-w-0 items-center gap-3">
      <div
        class="hidden h-12 items-center rounded-lg bg-theme-background-elevated p-1 sm:inline-flex"
      >
        <RouterLink
          :to="{ name: 'home' }"
          class="flex h-10 items-center gap-1.5 rounded-md px-4 text-sm font-semibold text-theme-text-muted outline-none transition-colors hover:text-theme-text focus-visible:ring-2 focus-visible:ring-theme-brand/60"
          :class="
            route.name === 'home' ||
            route.name === 'new' ||
            route.name === 'note'
              ? 'bg-theme-canvas text-theme-brand-strong shadow-sm'
              : ''
          "
        >
          <NotebookPen class="h-4 w-4" />
          {{ t("nav.notes") }}
        </RouterLink>
      </div>
    </div>

    <div class="ml-auto flex shrink-0 items-center gap-1.5">
      <UserMenu
        :username="globalStore.currentUser?.username"
        @sign-out="logOut"
      >
        <template #trigger>
          <Button
            variant="ghost"
            size="sm"
            class="h-9 gap-2 rounded-full px-1 sm:h-10 sm:pl-1.5 sm:pr-2"
            :title="t('nav.userMenu')"
            :aria-label="t('nav.userMenu')"
          >
            <span
              class="grid h-7 w-7 place-items-center rounded-full bg-theme-brand-soft text-xs font-semibold text-theme-brand-strong sm:h-7 sm:w-7"
            >
              {{ userInitial }}
            </span>
            <span
              class="hidden max-w-28 truncate text-sm text-theme-text-muted lg:inline"
              >{{ globalStore.currentUser?.username }}</span
            >
            <ChevronDown
              class="hidden h-3.5 w-3.5 text-theme-text-very-muted lg:block"
            />
          </Button>
        </template>
      </UserMenu>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ChevronDown, NotebookPen } from "lucide-vue-next";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { RouterLink, useRoute, useRouter } from "vue-router";

import Button from "../ui/Button.vue";
import { useGlobalStore } from "../../stores/globalStore";
import { resetAuthCheck } from "../../router";
import { clearStoredToken } from "../../api/tokenStorage";
import UserMenu from "./UserMenu.vue";

const { t } = useI18n();
const globalStore = useGlobalStore();
const route = useRoute();
const router = useRouter();
const userInitial = computed(
  () => globalStore.currentUser?.username?.charAt(0).toUpperCase() || "M",
);

function logOut() {
  clearStoredToken();
  globalStore.currentUser = null;
  resetAuthCheck();
  router.push({ name: "login" });
}
</script>
