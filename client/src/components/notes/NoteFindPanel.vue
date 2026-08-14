<template>
  <section
    class="absolute right-3 top-full z-40 mt-2 w-[min(36rem,calc(100vw-2rem))] rounded-xl border border-theme-border bg-theme-canvas p-5 text-theme-text shadow-2xl max-sm:fixed max-sm:inset-x-0 max-sm:bottom-0 max-sm:top-auto max-sm:mt-0 max-sm:w-full max-sm:rounded-b-none max-sm:border-x-0 max-sm:p-4 dark:bg-theme-background-elevated"
  >
    <div class="flex items-center gap-5 border-b border-theme-border pb-2">
      <button
        type="button"
        class="relative pb-1 text-lg font-semibold text-theme-text transition-colors duration-150 active:scale-[0.98] motion-reduce:transition-none"
        :class="
          activeTab === 'find'
            ? 'after:absolute after:inset-x-0 after:-bottom-[9px] after:h-1 after:rounded-full after:bg-theme-brand'
            : 'text-theme-text-muted'
        "
        @click="activeTab = 'find'"
      >
        {{ t("note.find") }}
      </button>
      <button
        type="button"
        class="relative pb-1 text-lg font-semibold text-theme-text-muted transition-colors duration-150 active:scale-[0.98] motion-reduce:transition-none"
        :class="
          activeTab === 'replace'
            ? 'text-theme-text after:absolute after:inset-x-0 after:-bottom-[9px] after:h-1 after:rounded-full after:bg-theme-brand'
            : ''
        "
        @click="activeTab = 'replace'"
      >
        {{ t("note.replace") }}
      </button>
      <Button
        variant="ghost"
        size="icon-sm"
        class="-mr-1 ml-auto"
        :title="t('common.close')"
        :aria-label="t('common.close')"
        @click="emit('close')"
      >
        <X class="h-5 w-5" />
      </Button>
    </div>

    <div class="mt-5 space-y-3">
      <Input
        ref="findInput"
        v-model="query"
        class="h-12 border-theme-brand/70 text-base"
        :placeholder="t('note.findPlaceholder')"
        @input="findFirst"
        @keydown.enter="findNext"
      />
      <Input
        v-if="activeTab === 'replace'"
        v-model="replacement"
        class="h-11 text-base"
        :placeholder="t('note.replacePlaceholder')"
        @keydown.enter="replaceNext"
      />
    </div>

    <div class="mt-3 flex items-center justify-between gap-3">
      <label class="flex items-center gap-2 text-sm text-theme-text-muted">
        <Checkbox v-model="caseSensitive" />
        {{ t("note.caseSensitive") }}
      </label>
      <div class="flex items-center gap-2">
        <span
          v-if="query"
          class="hidden text-sm tabular-nums text-theme-text-muted sm:inline"
          >{{ t("note.matchCount", matchCount) }}</span
        >
        <Button
          v-if="activeTab === 'replace'"
          variant="outline"
          size="sm"
          :disabled="!query"
          @click="replaceAll"
        >
          {{ t("note.replaceAll") }}
        </Button>
        <Button
          v-if="activeTab === 'replace'"
          variant="outline"
          size="sm"
          :disabled="!query"
          @click="replaceNext"
        >
          {{ t("note.replace") }}
        </Button>
        <Button
          variant="ghost"
          size="icon"
          :disabled="!query"
          :title="t('note.previousMatch')"
          :aria-label="t('note.previousMatch')"
          @click="findPrevious"
        >
          <ChevronLeft class="h-5 w-5" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          :disabled="!query"
          :title="t('note.nextMatch')"
          :aria-label="t('note.nextMatch')"
          @click="findNext"
        >
          <ChevronRight class="h-5 w-5" />
        </Button>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ChevronLeft, ChevronRight, X } from "lucide-vue-next";
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import Button from "../ui/Button.vue";
import Checkbox from "../ui/Checkbox.vue";
import Input from "../ui/Input.vue";

const emit = defineEmits(["close", "find", "replace", "replace-all"]);
const props = defineProps({
  matchCount: {
    type: Object,
    default: () => ({ current: 0, total: 0 }),
  },
});
const { t } = useI18n();
const activeTab = ref("find");
const caseSensitive = ref(false);
const findInput = ref();
const query = ref("");
const replacement = ref("");

watch(activeTab, () => nextTick(() => findInput.value?.focus()));
watch(caseSensitive, findFirst);

function findFirst() {
  if (!query.value) return;
  emit("find", {
    query: query.value,
    caseSensitive: caseSensitive.value,
    reset: true,
  });
}

function findNext() {
  if (!query.value) return;
  emit("find", {
    query: query.value,
    caseSensitive: caseSensitive.value,
    backwards: false,
  });
}

function findPrevious() {
  if (!query.value) return;
  emit("find", {
    query: query.value,
    caseSensitive: caseSensitive.value,
    backwards: true,
  });
}

function replaceNext() {
  if (!query.value) return;
  emit("replace", {
    query: query.value,
    replacement: replacement.value,
    caseSensitive: caseSensitive.value,
  });
}

function replaceAll() {
  if (!query.value) return;
  emit("replace-all", {
    query: query.value,
    replacement: replacement.value,
    caseSensitive: caseSensitive.value,
  });
}
</script>
