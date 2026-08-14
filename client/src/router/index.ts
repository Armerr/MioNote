import * as constants from "../utils/constants";

import { createRouter, createWebHistory } from "vue-router";

import { authCheck } from "../api";
import { i18n } from "../i18n";
import { clearStoredToken } from "../api/tokenStorage";

const router = createRouter({
  history: createWebHistory(""),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("../views/Home.vue"),
    },
    {
      path: "/login",
      name: "login",
      component: () => import("../views/LogIn.vue"),
      props: (route) => ({ redirect: route.query[constants.params.redirect] }),
    },
    {
      path: "/note/:title",
      name: "note",
      component: () => import("../views/Note.vue"),
      props: true,
    },
    {
      path: "/new",
      name: "new",
      component: () => import("../views/Note.vue"),
    },
    {
      path: "/search",
      name: "search",
      component: () => import("../views/SearchResults.vue"),
      props: (route) => ({
        searchTerm: route.query[constants.params.searchTerm],
        sortBy: Number(route.query[constants.params.sortBy]) || undefined,
      }),
    },
  ],
});

// Check the user is authenticated on first navigation (unless going to login)
let authChecked = false;
router.beforeEach(async (to) => {
  if (authChecked || to.name === "login") {
    return;
  }
  try {
    await authCheck();
    authChecked = true;
    return;
  } catch (error) {
    if (error.response && error.response.status === 401) {
      clearStoredToken();
      return {
        name: "login",
        query: { [constants.params.redirect]: to.fullPath },
      };
    }
  }
});

router.afterEach((to) => {
  let title = "MioNote";
  if (to.name === "note") {
    if (to.params.title) {
      title = `${to.params.title} - ${title}`;
    } else {
      title = `${i18n.global.t("nav.newNote")} - ${title}`;
    }
  }
  document.title = title;
});

export function resetAuthCheck() {
  authChecked = false;
}

export default router;
