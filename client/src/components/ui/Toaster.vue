<template>
  <ToastProvider :duration="5000" swipe-direction="right">
    <ToastRoot
      v-for="toast in toasts"
      :key="toast.id"
      :open="true"
      :class="toastClasses(toast.severity)"
      @update:open="(open) => !open && dismiss(toast.id)"
    >
      <div class="min-w-0 flex-1">
        <ToastTitle v-if="toast.summary" class="text-sm font-semibold">{{
          toast.summary
        }}</ToastTitle>
        <ToastDescription
          v-if="toast.detail"
          class="mt-0.5 text-sm text-theme-text-muted"
          >{{ toast.detail }}</ToastDescription
        >
      </div>
      <ToastClose
        class="grid h-7 w-7 shrink-0 place-items-center rounded text-theme-text-very-muted outline-none transition-colors hover:bg-theme-background-elevated hover:text-theme-text focus-visible:ring-2 focus-visible:ring-theme-brand/60"
      >
        <X class="h-4 w-4" />
      </ToastClose>
    </ToastRoot>
    <ToastViewport
      class="fixed bottom-4 right-4 z-[60] flex w-[calc(100%-2rem)] max-w-sm flex-col-reverse gap-2 outline-none sm:bottom-5 sm:right-5"
    />
  </ToastProvider>
</template>

<script setup lang="ts">
import { X } from "lucide-vue-next";
import {
  ToastClose,
  ToastDescription,
  ToastProvider,
  ToastRoot,
  ToastTitle,
  ToastViewport,
} from "reka-ui";

import { dismiss, toasts } from "../../composables/useToast";
import { cn } from "../../utils/cn";

function toastClasses(severity) {
  return cn(
    "flex w-full items-start gap-3 rounded-lg border bg-theme-canvas p-3 text-theme-text shadow-lg outline-none data-[state=open]:animate-[mionote-pop-in_180ms_cubic-bezier(0.16,1,0.3,1)] data-[state=closed]:animate-[mionote-pop-out_120ms_ease-in] data-[swipe=move]:translate-x-[var(--reka-toast-swipe-move-x)] data-[swipe=cancel]:translate-x-0 data-[swipe=cancel]:transition-transform data-[swipe=end]:animate-[mionote-pop-out_120ms_ease-in] motion-reduce:animate-none dark:bg-theme-background-elevated",
    severity === "success" && "border-theme-success/45",
    severity === "error" && "border-theme-danger/45",
    severity === "info" && "border-theme-brand/40",
  );
}
</script>
