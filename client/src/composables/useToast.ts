import { reactive } from "vue";

import type { Toast, ToastController, ToastOptions } from "../types";

const toasts = reactive<Toast[]>([]);
let nextToastId = 1;

function dismiss(id: number) {
  const index = toasts.findIndex((toast) => toast.id === id);
  if (index !== -1) toasts.splice(index, 1);
}

export function useToast(): ToastController {
  return {
    add(options: ToastOptions = {}) {
      const toast = {
        id: nextToastId++,
        summary: options.summary,
        detail: options.detail,
        severity: options.severity ?? "info",
      };

      toasts.push(toast);
      if (options.life !== 0) {
        window.setTimeout(() => dismiss(toast.id), options.life ?? 5000);
      }
      return toast;
    },
    remove: dismiss,
  };
}

export { dismiss, toasts };
