<template>
  <div
    class="flex h-full min-h-[420px] items-center justify-center px-5 py-8 sm:px-0 sm:py-0"
  >
    <div class="max-w-md text-center">
      <div
        class="mx-auto mb-6 grid h-12 w-12 place-items-center rounded-md bg-theme-brand-soft text-theme-brand-strong"
      >
        <NotebookPen class="h-5 w-5" />
      </div>
      <h1 class="text-2xl font-semibold">{{ t("home.heading") }}</h1>
      <p class="mt-3 leading-7 text-theme-text-muted">
        {{ t("home.description") }}
      </p>
      <Button size="lg" class="mt-7" @click="router.push({ name: 'new' })">
        <Plus class="h-4 w-4" />{{ t("home.createFirst") }}
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NotebookPen, Plus } from "lucide-vue-next";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import Button from "../components/ui/Button.vue";

const { t } = useI18n();
const router = useRouter();
</script>
