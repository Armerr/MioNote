<template>
  <Dialog
    v-model="isVisible"
    lightweight
    class="top-[18vh] w-[calc(100%-2rem)] max-w-md -translate-x-1/2 -translate-y-0 p-2 shadow-2xl sm:top-[22vh] sm:w-[min(32rem,calc(100%-2rem))] sm:p-3"
  >
    <div
      class="rounded-lg bg-theme-background-elevated px-2 ring-1 ring-theme-border/70"
    >
      <SearchInput
        ref="searchInput"
        large
        :placeholder="t('search.modalPlaceholder')"
        @search="toggleHandler"
        @keydown.esc="toggleHandler"
      />
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import Dialog from "../ui/Dialog.vue";
import SearchInput from "./SearchInput.vue";

const isVisible = defineModel({ type: Boolean });
const { t } = useI18n();
const searchInput = ref();

watch(isVisible, async (visible) => {
  if (!visible) return;
  await nextTick();
  window.setTimeout(() => searchInput.value?.focus(), 130);
});

function toggleHandler() {
  isVisible.value = !isVisible.value;
}
</script>
