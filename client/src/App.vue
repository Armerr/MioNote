<template>
  <LoadingIndicator
    ref="loadingIndicator"
    class="flex h-dvh flex-col overflow-hidden bg-theme-background"
  >
    <Toaster />
    <template v-if="showWorkspace">
      <MobileNoteSheet ref="mobileNoteSheet" v-model="isNoteSheetVisible" />
      <NavBar />
      <div class="flex min-h-0 flex-1">
        <div
          class="hidden shrink-0 lg:flex"
          :style="{ width: `${sidebarWidth}px` }"
        >
          <NoteSidebar ref="noteSidebar" />
        </div>
        <button
          class="hidden w-2 shrink-0 cursor-col-resize items-stretch border-x border-transparent bg-transparent transition-colors hover:border-theme-brand/30 hover:bg-theme-brand/10 focus-visible:border-theme-brand/60 focus-visible:bg-theme-brand/10 lg:flex"
          type="button"
          role="separator"
          :aria-label="t('sidebar.resize')"
          :aria-valuemin="SIDEBAR_MIN_WIDTH"
          :aria-valuemax="SIDEBAR_MAX_WIDTH"
          :aria-valuenow="sidebarWidth"
          tabindex="0"
          @pointerdown="startSidebarResize"
          @keydown.left.prevent="resizeSidebarBy(-16)"
          @keydown.right.prevent="resizeSidebarBy(16)"
        ></button>
        <main
          class="xl:px-9 min-w-0 flex-1 overflow-y-auto bg-theme-canvas pb-0 sm:px-8 sm:pb-24 sm:pt-7 lg:px-8 lg:py-6"
        >
          <RouterView @open-search="focusWorkspaceSearch" />
        </main>
      </div>
    </template>
    <RouterView v-else />
  </LoadingIndicator>
</template>

<script setup>
import Mousetrap from "mousetrap";
import "mousetrap/plugins/global-bind/mousetrap-global-bind";
import { computed, nextTick, onBeforeUnmount, ref } from "vue";
import { useI18n } from "vue-i18n";
import { RouterView, useRoute } from "vue-router";

import { apiErrorHandler, getConfig, getCurrentUser } from "./api";
import LoadingIndicator from "./components/common/LoadingIndicator.vue";
import MobileNoteSheet from "./components/layout/MobileNoteSheet.vue";
import NavBar from "./components/layout/NavBar.vue";
import NoteSidebar from "./components/layout/NoteSidebar.vue";
import Toaster from "./components/ui/Toaster.vue";
import { useToast } from "./composables/useToast";
import router from "./router";
import { useGlobalStore } from "./stores/globalStore";
import { loadTheme } from "./utils/helpers";

const globalStore = useGlobalStore();
const { t } = useI18n();
const mobileNoteSheet = ref();
const noteSidebar = ref();
const isNoteSheetVisible = ref(false);
const SIDEBAR_MIN_WIDTH = 280;
const SIDEBAR_MAX_WIDTH = 560;
const SIDEBAR_DEFAULT_WIDTH = 390;
const sidebarWidth = ref(loadSidebarWidth());
let sidebarResizing = false;
const loadingIndicator = ref();
const route = useRoute();
const toast = useToast();

// '/' to search
Mousetrap.bind("/", () => {
  if (route.name !== "login") {
    focusWorkspaceSearch();
    return false;
  }
});

// 'CTRL + ALT/OPT + N' to create new note
Mousetrap.bindGlobal("ctrl+alt+n", () => {
  if (route.name !== "login") {
    router.push({ name: "new" });
    return false;
  }
});

// 'CTRL + ALT/OPT + H' to go to home
Mousetrap.bindGlobal("ctrl+alt+h", () => {
  if (route.name !== "login") {
    router.push({ name: "home" });
    return false;
  }
});

getConfig()
  .then((data) => {
    globalStore.config = data;
    loadingIndicator.value.setLoaded();
    getCurrentUser()
      .then((user) => {
        globalStore.currentUser = user;
      })
      .catch((error) => {
        globalStore.currentUser = null;
        apiErrorHandler(error, toast);
      });
  })
  .catch((error) => {
    apiErrorHandler(error, toast);
    loadingIndicator.value.setFailed();
  });

const showWorkspace = computed(() => {
  return route.name !== "login";
});

async function focusWorkspaceSearch() {
  if (window.matchMedia("(min-width: 1024px)").matches) {
    noteSidebar.value?.focusSearch();
    return;
  }

  isNoteSheetVisible.value = true;
  await nextTick();
  mobileNoteSheet.value?.focusSearch();
}

function loadSidebarWidth() {
  const stored = Number(localStorage.getItem("mionote.sidebar-width"));
  return Number.isFinite(stored)
    ? Math.min(SIDEBAR_MAX_WIDTH, Math.max(SIDEBAR_MIN_WIDTH, stored))
    : SIDEBAR_DEFAULT_WIDTH;
}

function setSidebarWidth(width) {
  sidebarWidth.value = Math.min(
    SIDEBAR_MAX_WIDTH,
    Math.max(SIDEBAR_MIN_WIDTH, width),
  );
  localStorage.setItem("mionote.sidebar-width", String(sidebarWidth.value));
}

function resizeSidebarBy(delta) {
  setSidebarWidth(sidebarWidth.value + delta);
}

function startSidebarResize(event) {
  if (event.pointerType === "mouse" && event.button !== 0) return;
  sidebarResizing = true;
  event.currentTarget?.setPointerCapture?.(event.pointerId);
  document.body.classList.add("mionote-resizing-sidebar");
  window.addEventListener("pointermove", onSidebarPointerMove);
  window.addEventListener("pointerup", stopSidebarResize, { once: true });
}

function onSidebarPointerMove(event) {
  if (!sidebarResizing) return;
  setSidebarWidth(event.clientX);
}

function stopSidebarResize() {
  sidebarResizing = false;
  document.body.classList.remove("mionote-resizing-sidebar");
  window.removeEventListener("pointermove", onSidebarPointerMove);
}

onBeforeUnmount(stopSidebarResize);

loadTheme();
</script>

<style>
body.mionote-resizing-sidebar,
body.mionote-resizing-sidebar * {
  cursor: col-resize !important;
  user-select: none !important;
}
</style>
