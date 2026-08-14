<template>
  <div class="relative w-full">
    <!-- Input -->
    <div
      class="flex w-full items-center gap-2"
      :class="{ 'h-10': !large, 'h-14': large }"
    >
      <Search
        class="ml-3 shrink-0 text-theme-text-muted"
        :class="large ? 'h-5 w-5' : 'h-4 w-4'"
      />
      <Input
        type="text"
        ref="input"
        v-model="searchTerm"
        class="h-full border-0 bg-transparent px-0 pr-3 text-base shadow-none focus:border-0 focus:ring-0"
        :placeholder="resolvedPlaceholder"
        @keydown="keydownHandler"
        @keyup="stateChangeHandler"
        @click="stateChangeHandler"
        @blur="tagMenuVisible = false"
        @keydown.down.prevent
        @keydown.up.prevent
      />
      <!-- Note: Default behaviour for up and down keys is prevented to stop cursor moving when tag menu is navigated. -->
    </div>

    <!-- Tag Menu -->
    <div
      v-if="tagMenuVisible"
      class="absolute z-20 mt-2 max-h-64 w-full overflow-scroll rounded-md border border-theme-border bg-theme-canvas p-1 shadow-lg dark:bg-theme-background-elevated"
    >
      <p
        v-for="(tag, index) in tagMatches"
        ref="tagMenuItems"
        class="cursor-pointer rounded px-2 py-1 hover:bg-theme-background-elevated"
        :class="{ 'bg-theme-background-elevated': index === tagMenuIndex }"
        @click="tagChosen(tag)"
        @mousedown.prevent
      >
        <!-- Note: Default behaviour for mouse down is prevented to stop focus moving to menu on click. -->
        {{ tag }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Search } from "lucide-vue-next";
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { apiErrorHandler, getTags } from "../../api";
import Input from "../ui/Input.vue";
import { useToast } from "../../composables/useToast";
import * as constants from "../../utils/constants";
import { getToastOptions } from "../../utils/helpers";

const props = defineProps({
  initialSearchTerm: { type: String, default: "" },
  large: Boolean,
  placeholder: { type: String, default: "" },
});
const emit = defineEmits(["search"]);

const { t } = useI18n();
const input = ref();
const router = useRouter();
const searchTerm = ref(props.initialSearchTerm);
const toast = useToast();
let tags = null;
const tagMatches = ref([]);
const tagMenuItems = ref([]);
const tagMenuIndex = ref(0);
const tagMenuVisible = ref(false);
const resolvedPlaceholder = computed(
  () => props.placeholder || t("search.placeholder"),
);

function keydownHandler(event) {
  // Tag Menu Open
  if (tagMenuVisible.value) {
    if (event.key === "ArrowDown") {
      tagMenuIndex.value = Math.min(
        tagMenuIndex.value + 1,
        tagMatches.value.length - 1,
      );
      tagMenuItems.value[tagMenuIndex.value].scrollIntoView({
        block: "nearest",
      });
    } else if (event.key === "ArrowUp") {
      tagMenuIndex.value = Math.max(tagMenuIndex.value - 1, 0);
      tagMenuItems.value[tagMenuIndex.value].scrollIntoView({
        block: "nearest",
      });
    } else if (event.key === "Enter") {
      tagChosen(tagMatches.value[tagMenuIndex.value]);
    } else if (event.key === "Escape") {
      tagMenuVisible.value = false;
      event.stopPropagation(); // Prevent the modal from closing when the tag menu is open.
    }
  }
  // Tag Menu Closed
  else if (event.key === "Enter") {
    search();
  }
}

function tagChosen(tag) {
  replaceWordOnCursor(tag);
  tagMenuVisible.value = false;
}

function search() {
  if (searchTerm.value) {
    router.push({
      name: "search",
      query: { [constants.params.searchTerm]: searchTerm.value },
    });
    emit("search");
  } else {
    toast.add(
      getToastOptions(t("search.missingTerm"), t("common.error"), "error"),
    );
  }
}

function stateChangeHandler() {
  const wordOnCursor = getWordOnCursor();
  if (wordOnCursor.charAt(0) !== "#") {
    tagMenuVisible.value = false;
    tagMatches.value = [];
  } else {
    // All tags are stored in lowercase, so we can do a case-insensitive search.
    filterTagMatches(wordOnCursor.toLowerCase());
  }
}

async function filterTagMatches(input) {
  if (tags === null) {
    try {
      tags = await getTags();
    } catch (error) {
      tags = [];
      apiErrorHandler(error, toast);
    }
    tags = tags.map((tag) => `#${tag}`);
  }
  const currentTagMatchCount = tagMatches.value.length;
  tagMatches.value = tags.filter(
    (tag) => tag.startsWith(input) && tag !== input,
  );
  if (
    currentTagMatchCount !== tagMatches.value.length &&
    tagMatches.value.length > 0
  ) {
    tagMenuIndex.value = 0;
    tagMenuVisible.value = true;
  } else if (tagMatches.value.length === 0) {
    tagMenuVisible.value = false;
  }
}

// Helpers

/**
 * Returns the word that the cursor is currently on.
 * @returns {Object} An object containing the start and end indices of the word.
 */
function getWordOnCursorPosition() {
  const cursorPosition = input.value.selectionStart;
  const wordStart = Math.max(
    searchTerm.value.lastIndexOf(" ", cursorPosition - 1) + 1,
    0,
  );
  let wordEnd = searchTerm.value.indexOf(" ", cursorPosition);
  if (wordEnd === -1) {
    // If there is no space after the cursor, then the word ends at the end of the input.
    wordEnd = searchTerm.value.length;
  }
  return { start: wordStart, end: wordEnd };
}

/**
 * Retrieves the word at the current cursor position in the search term.
 * @returns {string} The word at the cursor position.
 */
function getWordOnCursor() {
  const { start, end } = getWordOnCursorPosition();
  return searchTerm.value.substring(start, end);
}

/**
 * Replaces the word at the cursor position with the given replacement.
 * @param {string} replacement The word to replace the current word with.
 */
function replaceWordOnCursor(replacement) {
  const { start, end } = getWordOnCursorPosition();
  searchTerm.value =
    searchTerm.value.substring(0, start) +
    replacement +
    searchTerm.value.substring(end);
}

watch(
  () => props.initialSearchTerm,
  () => {
    searchTerm.value = props.initialSearchTerm;
  },
);

function focus() {
  input.value?.focus();
}

defineExpose({ focus });
</script>
