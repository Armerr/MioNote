import colors from "tailwindcss/colors";
import type { Config } from "tailwindcss";

export default {
  content: ["./client/index.html", "./client/src/**/*.{ts,vue}"],
  darkMode: "selector",
  theme: {
    fontFamily: {
      sans: ["PingFang SC", "Microsoft YaHei", "Poppins", "sans-serif"],
    },
    screens: {
      sm: "640px",
      md: "768px",
      lg: "1024px",
    },
    extend: {
      colors: {
        // Dynamic
        "theme-brand": "rgb(var(--theme-brand) / <alpha-value>)",
        "theme-background": "rgb(var(--theme-background) / <alpha-value>)",
        "theme-background-elevated":
          "rgb(var(--theme-background-elevated) / <alpha-value>)",
        "theme-text": "rgb(var(--theme-text) / <alpha-value>)",
        "theme-text-muted": "rgb(var(--theme-text-muted) / <alpha-value>)",
        "theme-text-very-muted":
          "rgb(var(--theme-text-very-muted) / <alpha-value>)",
        "theme-shadow": "rgb(var(--theme-shadow) / <alpha-value>)",
        "theme-border": "rgb(var(--theme-border) / <alpha-value>)",
        "theme-header": "rgb(var(--theme-header) / <alpha-value>)",
        "theme-canvas": "rgb(var(--theme-canvas) / <alpha-value>)",
        "theme-sidebar": "rgb(var(--theme-sidebar) / <alpha-value>)",
        "theme-sidebar-active":
          "rgb(var(--theme-sidebar-active) / <alpha-value>)",
        "theme-sidebar-selected":
          "rgb(var(--theme-sidebar-selected) / <alpha-value>)",
        "theme-brand-soft": "rgb(var(--theme-brand-soft) / <alpha-value>)",
        "theme-brand-strong": "rgb(var(--theme-brand-strong) / <alpha-value>)",
        "theme-brand-contrast":
          "rgb(var(--theme-brand-contrast) / <alpha-value>)",
        "theme-auth-panel": "rgb(var(--theme-auth-panel) / <alpha-value>)",
        "theme-auth-text": "rgb(var(--theme-auth-text) / <alpha-value>)",
        "theme-auth-muted": "rgb(var(--theme-auth-muted) / <alpha-value>)",
        // Static
        "theme-success": colors.emerald[600],
        "theme-danger": colors.rose[600],
      },
    },
  },
  plugins: [],
} satisfies Config;
