<template>
  <div
    class="grid h-full min-h-dvh bg-theme-background lg:grid-cols-[minmax(360px,42%)_1fr]"
  >
    <section
      class="relative hidden overflow-hidden bg-theme-auth-panel px-12 py-14 lg:flex lg:flex-col"
    >
      <Logo />
      <div class="my-auto max-w-sm">
        <div class="mb-6 h-1.5 w-12 bg-theme-brand"></div>
        <h1 class="text-4xl font-semibold leading-tight text-theme-auth-text">
          {{ t("auth.welcome") }}
        </h1>
        <p class="mt-5 text-base leading-7 text-theme-auth-muted">
          {{ t("auth.subtitle") }}
        </p>
      </div>
      <p class="text-xs text-theme-auth-muted">MioNote</p>
    </section>

    <main class="flex items-center justify-center px-5 py-8 sm:px-12 sm:py-10">
      <div class="w-full max-w-[360px]">
        <div class="mb-8 sm:mb-12 lg:hidden"><Logo /></div>
        <div class="mb-7 flex items-center justify-between">
          <div>
            <p
              class="text-xs font-semibold uppercase text-theme-text-very-muted"
            >
              MioNote
            </p>
            <h2
              class="mt-2 text-[28px] font-semibold leading-tight sm:text-3xl"
            >
              {{ mode === "login" ? t("auth.login") : t("auth.createAccount") }}
            </h2>
          </div>
          <LanguageToggle />
        </div>

        <div
          class="mb-7 inline-flex rounded-md border border-theme-border bg-theme-background-elevated p-1 text-sm font-semibold"
        >
          <Button
            size="sm"
            :variant="mode === 'login' ? 'outline' : 'ghost'"
            class="border-0"
            @click="mode = 'login'"
          >
            {{ t("auth.login") }}
          </Button>
          <Button
            v-if="globalStore.config.registrationOpen"
            size="sm"
            :variant="mode === 'register' ? 'outline' : 'ghost'"
            class="border-0"
            @click="mode = 'register'"
          >
            {{ t("auth.register") }}
          </Button>
        </div>

        <form class="flex flex-col gap-3" @submit.prevent="submit">
          <label class="text-sm font-medium" for="username">{{
            t("auth.username")
          }}</label>
          <Input
            v-model="username"
            id="username"
            :placeholder="t('auth.username')"
            autocomplete="username"
            required
          />
          <p
            v-if="mode === 'register'"
            class="-mt-1 text-xs text-theme-text-very-muted"
          >
            {{ t("auth.usernameHint") }}
          </p>

          <label class="mt-2 text-sm font-medium" for="password">{{
            t("auth.password")
          }}</label>
          <Input
            v-model="password"
            id="password"
            :placeholder="t('auth.password')"
            type="password"
            :autocomplete="
              mode === 'login' ? 'current-password' : 'new-password'
            "
            required
          />
          <p
            v-if="mode === 'register'"
            class="-mt-1 text-xs text-theme-text-very-muted"
          >
            {{ t("auth.passwordHint") }}
          </p>

          <template v-if="mode === 'register'">
            <label
              class="mt-2 text-sm font-medium"
              for="password-confirmation"
              >{{ t("auth.confirmPassword") }}</label
            >
            <Input
              v-model="passwordConfirmation"
              id="password-confirmation"
              :placeholder="t('auth.confirmPassword')"
              type="password"
              autocomplete="new-password"
              required
            />
          </template>

          <label
            v-if="mode === 'login'"
            class="mt-2 flex cursor-pointer items-center gap-2 text-sm text-theme-text-muted"
            for="remember-me"
          >
            <Checkbox id="remember-me" v-model="rememberMe" />
            {{ t("auth.rememberMe") }}
          </label>

          <Button type="submit" size="lg" class="mt-4 w-full">
            <LogIn class="h-4 w-4" />
            {{ mode === "login" ? t("auth.login") : t("auth.createAccount") }}
          </Button>
        </form>

        <p
          v-if="globalStore.config.registrationOpen"
          class="mt-7 text-sm text-theme-text-muted"
        >
          {{ mode === "login" ? t("auth.noAccount") : t("auth.hasAccount") }}
          <Button
            type="button"
            variant="link"
            size="sm"
            class="ml-1 font-semibold"
            @click="toggleMode"
          >
            {{ mode === "login" ? t("auth.register") : t("auth.login") }}
          </Button>
        </p>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { LogIn } from "lucide-vue-next";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import {
  apiErrorHandler,
  getCurrentUser,
  getToken,
  registerAccount,
} from "../api";
import LanguageToggle from "../components/common/LanguageToggle.vue";
import Logo from "../components/common/Logo.vue";
import Button from "../components/ui/Button.vue";
import Checkbox from "../components/ui/Checkbox.vue";
import Input from "../components/ui/Input.vue";
import { useToast } from "../composables/useToast";
import { useGlobalStore } from "../stores/globalStore";
import { getToastOptions } from "../utils/helpers";
import { storeToken } from "../api/tokenStorage";

const props = defineProps({ redirect: String });

const { t } = useI18n();
const globalStore = useGlobalStore();
const mode = ref("login");
const password = ref("");
const passwordConfirmation = ref("");
const rememberMe = ref(false);
const router = useRouter();
const toast = useToast();
const username = ref("");

function toggleMode() {
  mode.value = mode.value === "login" ? "register" : "login";
}

async function submit() {
  if (
    mode.value === "register" &&
    password.value !== passwordConfirmation.value
  ) {
    toast.add(
      getToastOptions(
        t("auth.passwordMismatch"),
        t("auth.registerFailed"),
        "error",
      ),
    );
    return;
  }
  try {
    const accessToken =
      mode.value === "login"
        ? await getToken(username.value, password.value)
        : await registerAccount(username.value, password.value);
    storeToken(accessToken, mode.value === "login" && rememberMe.value);
    globalStore.currentUser = await getCurrentUser();
    router.push(props.redirect || { name: "home" });
  } catch (error) {
    password.value = "";
    passwordConfirmation.value = "";
    if ([400, 401, 409].includes(error.response?.status)) {
      toast.add(
        getToastOptions(
          mode.value === "login"
            ? t("auth.invalidCredentials")
            : error.response.data?.detail || t("auth.registerFailed"),
          mode.value === "login"
            ? t("auth.loginFailed")
            : t("auth.registerFailed"),
          "error",
        ),
      );
    } else {
      apiErrorHandler(error, toast);
    }
  }
}
</script>
