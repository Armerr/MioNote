<template>
  <div ref="viewerElement"></div>
</template>

<script setup lang="ts">
import Viewer from "@toast-ui/editor/dist/toastui-editor-viewer";
import "@toast-ui/editor/dist/i18n/zh-cn";
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import baseOptions from "./baseOptions";
import extendedAutolinks from "./extendedAutolinks";

const props = defineProps({
  initialValue: String,
});

const { locale } = useI18n();
const viewerElement = ref();

onMounted(() => {
  new Viewer({
    ...baseOptions,
    extendedAutolinks,
    el: viewerElement.value,
    initialValue: props.initialValue,
    language: locale.value,
  });
});
</script>

<style>
@import "@toast-ui/editor/dist/toastui-editor-viewer.css";
@import "prismjs/themes/prism.css";
@import "@toast-ui/editor-plugin-code-syntax-highlight/dist/toastui-editor-plugin-code-syntax-highlight.css";
@import "./toastui-editor-overrides.scss";
</style>
