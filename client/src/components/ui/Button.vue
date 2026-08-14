<template>
  <button
    ref="buttonElement"
    v-bind="buttonAttrs"
    :class="buttonClasses"
    :type="type"
    @mouseenter="showTooltip"
    @mouseleave="hideTooltip"
    @focus="showTooltip"
    @blur="hideTooltip"
  >
    <slot />
  </button>
  <Teleport to="body">
    <Transition name="mionote-tooltip">
      <span
        v-if="tooltipVisible"
        role="tooltip"
        class="pointer-events-none fixed z-[100] flex -translate-x-1/2 items-center gap-2 whitespace-nowrap rounded-md bg-zinc-800 px-2.5 py-1 text-xs font-medium leading-5 text-white shadow-lg dark:bg-zinc-100 dark:text-zinc-900"
        :style="tooltipPosition"
      >
        {{ tooltip }}
        <kbd
          v-if="tooltipShortcut"
          class="font-sans text-[11px] text-zinc-300 dark:text-zinc-600"
          >{{ tooltipShortcut }}</kbd
        >
      </span>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { cva } from "class-variance-authority";
import { computed, onBeforeUnmount, ref, useAttrs } from "vue";

import { cn } from "../../utils/cn";
import type { ButtonSize, ButtonType, ButtonVariant } from "../../types";

defineOptions({ inheritAttrs: false });

const props = withDefaults(
  defineProps<{
    variant?: ButtonVariant;
    size?: ButtonSize;
    type?: ButtonType;
    tooltipShortcut?: string;
  }>(),
  {
    variant: "ghost",
    size: "default",
    type: "button",
    tooltipShortcut: "",
  },
);

const attrs = useAttrs();
const tooltipShortcut = computed(() => props.tooltipShortcut);
const buttonElement = ref<HTMLButtonElement | null>(null);
const tooltipVisible = ref(false);
const tooltipPosition = ref({ left: "0px", top: "0px" });
let tooltipTimer: number | undefined;
const buttonVariants = cva(
  "relative inline-flex shrink-0 items-center justify-center gap-2 whitespace-nowrap overflow-visible rounded-md text-sm font-medium transition-[transform,background-color,border-color,color,box-shadow] duration-150 ease-out outline-none hover:brightness-[0.99] active:scale-[0.97] focus-visible:ring-2 focus-visible:ring-theme-brand/70 focus-visible:ring-offset-2 focus-visible:ring-offset-theme-canvas motion-reduce:transition-none disabled:pointer-events-none disabled:opacity-45",
  {
    variants: {
      variant: {
        default:
          "bg-theme-brand text-theme-brand-contrast shadow-sm hover:bg-theme-brand-strong hover:shadow-md active:translate-y-px",
        secondary:
          "bg-theme-background-elevated text-theme-text hover:bg-theme-sidebar-active",
        outline:
          "border border-theme-border bg-theme-canvas text-theme-text hover:bg-theme-background-elevated",
        ghost:
          "text-theme-text-muted hover:bg-theme-background-elevated hover:text-theme-text",
        destructive:
          "bg-theme-danger text-white shadow-sm hover:bg-theme-danger/90",
        success:
          "bg-theme-success text-white shadow-sm hover:bg-theme-success/90",
        link: "h-auto px-0 py-0 text-theme-brand-strong underline-offset-4 hover:underline",
      },
      size: {
        default: "h-11 px-4 py-2 sm:h-10",
        sm: "h-10 px-3 text-xs sm:h-8",
        lg: "h-12 px-5 sm:h-11",
        icon: "h-11 w-11 sm:h-10 sm:w-10",
        "icon-sm": "h-10 w-10 sm:h-8 sm:w-8",
        "icon-lg": "h-12 w-12 sm:h-11 sm:w-11",
      },
    },
  },
);

const buttonClasses = computed(() =>
  cn(buttonVariants({ variant: props.variant, size: props.size }), attrs.class),
);
const tooltip = computed(() => attrs.title || "");
const buttonAttrs = computed(() => {
  const { class: _class, title: _title, ...rest } = attrs;
  return rest;
});

function showTooltip() {
  if (!tooltip.value || attrs.disabled) return;

  window.clearTimeout(tooltipTimer);
  tooltipTimer = window.setTimeout(() => {
    const rect = buttonElement.value?.getBoundingClientRect();
    if (!rect) return;
    tooltipPosition.value = {
      left: `${rect.left + rect.width / 2}px`,
      top: `${rect.bottom + 7}px`,
    };
    tooltipVisible.value = true;
  }, 80);
}

function hideTooltip() {
  window.clearTimeout(tooltipTimer);
  tooltipVisible.value = false;
}

onBeforeUnmount(() => window.clearTimeout(tooltipTimer));
</script>

<style scoped>
.mionote-tooltip-enter-active,
.mionote-tooltip-leave-active {
  transition:
    opacity 100ms ease,
    transform 100ms ease;
}

.mionote-tooltip-enter-from,
.mionote-tooltip-leave-to {
  opacity: 0;
  transform: translate(-50%, -2px);
}
</style>
