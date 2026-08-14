<template>
  <section
    class="absolute right-3 top-full z-40 mt-2 w-[min(40rem,calc(100vw-2rem))] rounded-xl border border-theme-border bg-theme-canvas p-5 text-theme-text shadow-2xl max-sm:fixed max-sm:inset-x-0 max-sm:bottom-0 max-sm:top-auto max-sm:mt-0 max-sm:w-full max-sm:rounded-b-none max-sm:border-x-0 max-sm:p-4 dark:bg-theme-background-elevated"
  >
    <div class="flex items-center justify-between">
      <h2 class="text-lg font-semibold">{{ t("note.paperStyle") }}</h2>
      <Button
        variant="ghost"
        size="icon-sm"
        :title="t('common.close')"
        :aria-label="t('common.close')"
        @click="emit('close')"
      >
        <X class="h-5 w-5" />
      </Button>
    </div>

    <div class="mt-5 grid grid-cols-5 gap-2 sm:flex sm:gap-3">
      <button
        v-for="option in paperOptions"
        :key="option.id"
        type="button"
        class="grid h-12 w-full place-items-center rounded-lg border outline-none transition-transform hover:-translate-y-0.5 focus-visible:ring-2 focus-visible:ring-theme-brand/70 sm:h-20 sm:w-20"
        :class="
          paper === option.id
            ? 'border-theme-brand shadow-md'
            : 'border-theme-border'
        "
        :style="{ backgroundColor: option.color }"
        :title="t(option.label)"
        :aria-label="t(option.label)"
        @click="emit('update:paper', option.id)"
      >
        <Check
          v-if="paper === option.id"
          class="h-5 w-5 text-theme-text sm:h-7 sm:w-7"
          :stroke-width="3"
        />
      </button>
    </div>

    <div
      class="mt-6 grid grid-cols-[6.5rem_1fr] items-center gap-x-3 gap-y-5 sm:mt-8 sm:grid-cols-[8rem_1fr] sm:gap-x-4 sm:gap-y-6"
    >
      <label class="text-sm font-medium sm:text-base">{{
        t("note.paperTexture")
      }}</label>
      <input
        :value="paperTexture"
        class="accent-theme-brand"
        type="range"
        min="0"
        max="100"
        step="5"
        @input="emit('update:paper-texture', getRangeValue($event))"
      />

      <label class="text-sm font-medium sm:text-base">{{
        t("note.pageMargin")
      }}</label>
      <div>
        <input
          :value="pageMargin"
          class="w-full accent-theme-brand"
          type="range"
          min="16"
          max="64"
          step="4"
          @input="emit('update:page-margin', getRangeValue($event))"
        />
        <div
          class="mt-1.5 flex justify-between text-sm text-theme-text-very-muted"
        >
          <span>{{ t("note.marginSmall") }}</span>
          <span>{{ t("note.marginStandard") }}</span>
          <span>{{ t("note.marginLarge") }}</span>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { Check, X } from "lucide-vue-next";
import { useI18n } from "vue-i18n";

import Button from "../ui/Button.vue";

defineProps({
  paper: { type: String, default: "plain" },
  paperTexture: { type: Number, default: 0 },
  pageMargin: { type: Number, default: 24 },
});

const emit = defineEmits([
  "close",
  "update:paper",
  "update:paper-texture",
  "update:page-margin",
]);
const { t } = useI18n();
const paperOptions = [
  { id: "plain", color: "#ffffff", label: "note.paperPlain" },
  { id: "mist", color: "#f5f5f4", label: "note.paperMist" },
  { id: "cream", color: "#fffde8", label: "note.paperCream" },
  { id: "peach", color: "#fff2e9", label: "note.paperPeach" },
  { id: "sky", color: "#edf7ff", label: "note.paperSky" },
];

function getRangeValue(event: Event) {
  return Number((event.target as HTMLInputElement).value);
}
</script>
