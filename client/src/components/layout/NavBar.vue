<template>
  <nav
    class="hidden h-[70px] shrink-0 items-center border-b border-theme-border bg-theme-header px-6 sm:flex"
  >
    <div class="flex min-w-0 items-center gap-3">
      <div
        class="inline-flex h-12 items-center rounded-lg bg-theme-background-elevated p-1"
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
  </nav>
</template>

<script setup lang="ts">
import { NotebookPen } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { RouterLink, useRoute } from "vue-router";

const { t } = useI18n();
const route = useRoute();
</script>
