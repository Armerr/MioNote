<template>
  <div class="flex items-center gap-2">
    <img
      class="h-9 w-9 shrink-0"
      src="/favicon.svg"
      alt=""
      aria-hidden="true"
    />
    <span
      class="text-lg font-semibold text-theme-text"
      :class="{ 'hidden sm:block': responsive }"
      >{{ t("common.appName") }}</span
    >
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

defineProps({
  responsive: {
    type: Boolean,
    default: false,
  },
});

const { t } = useI18n();
</script>
