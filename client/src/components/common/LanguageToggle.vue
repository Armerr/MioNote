<template>
  <SegmentedControl
    :model-value="locale"
    :items="items"
    :aria-label="t('nav.language')"
    @update:model-value="setLocale"
  />
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

import SegmentedControl from "../ui/SegmentedControl.vue";
import { setLocale } from "../../i18n";

const { locale, t } = useI18n();
const items = [
  { value: "zh-CN", label: "中", title: "中文" },
  { value: "en-US", label: "EN", title: "English" },
];
</script>
