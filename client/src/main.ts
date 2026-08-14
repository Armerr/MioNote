import App from "./App.vue";
import { createApp } from "vue";
import { createPinia } from "pinia";
import { loadStoredToken } from "./api/tokenStorage";
import { i18n } from "./i18n";
import router from "./router";
import "./style.css";

const app = createApp(App);
const pinia = createPinia();

app.use(router);
app.use(pinia);
app.use(i18n);

// Custom v-focus directive to focus on an element when mounted
app.directive("focus", {
  mounted(el) {
    el.focus();
  },
});

loadStoredToken();

app.mount("#app");
