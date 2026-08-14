import { defineStore } from "pinia";
import { ref } from "vue";

import type { AppConfig, CurrentUser } from "../types";

export const useGlobalStore = defineStore("global", () => {
  const config = ref<AppConfig>({
    authType: "password",
    registrationOpen: true,
  });
  const currentUser = ref<CurrentUser | null>(null);

  return { config, currentUser };
});
