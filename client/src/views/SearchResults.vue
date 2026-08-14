<template>
  <div class="mx-auto flex h-full max-w-4xl flex-col px-4 pt-5 sm:px-0 sm:pt-0">
    <!-- Search Input -->
    <SearchInput :initialSearchTerm="props.searchTerm" class="mb-2" />

    <LoadingIndicator ref="loadingIndicator" class="flex-1">
      <!-- Sort By -->
      <div class="mb-2 flex justify-end">
        <ActionMenu :items="menuItems">
          <template #trigger>
            <Button variant="outline" size="sm">
              <ArrowDownUp class="h-4 w-4" />
              {{ t("search.sortBy", { name: sortByName }) }}
            </Button>
          </template>
        </ActionMenu>
      </div>

      <!-- Search Results -->
      <div
        v-for="result in results"
        class="mb-2 cursor-pointer rounded-lg border border-transparent px-4 py-3 transition-colors hover:bg-theme-background-elevated"
      >
        <RouterLink :to="{ name: 'note', params: { title: result.title } }">
          <!-- Title and Tags -->
          <div>
            <span v-html="resultListTitle(result)" class="mr-2"></span>
            <Tag v-for="tag in result.tagMatches" :tag="tag" class="mr-1" />
          </div>
          <!-- Last Modified and Content Highlights -->
          <div>
            <span class="text-theme-text-muted">{{
              result.lastModifiedAsString
            }}</span>
            <span v-if="result.contentHighlights"> - </span>
            <span
              v-html="result.contentHighlights"
              class="text-theme-text-muted"
            ></span>
          </div>
        </RouterLink>
      </div>
    </LoadingIndicator>
  </div>
</template>

<script setup lang="ts">
import { ArrowDownUp, Search } from "lucide-vue-next";
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { apiErrorHandler, getNotes } from "../api";
import LoadingIndicator from "../components/common/LoadingIndicator.vue";
import Tag from "../components/common/Tag.vue";
import ActionMenu from "../components/ui/ActionMenu.vue";
import Button from "../components/ui/Button.vue";
import { useToast } from "../composables/useToast";
import { params, searchSortOptions } from "../utils/constants";
import { defaultNoteTitle } from "../utils/constants";
import SearchInput from "../components/search/SearchInput.vue";

const props = defineProps({
  searchTerm: String,
  sortBy: {
    type: Number,
    default: searchSortOptions.score,
  },
});

const { t } = useI18n();
const loadingIndicator = ref();
const results = ref([]);
const router = useRouter();
const toast = useToast();

const sortByName = computed(() => {
  const sortOptionNames = {
    [searchSortOptions.title]: t("search.title"),
    [searchSortOptions.lastModified]: t("search.lastModified"),
    [searchSortOptions.score]: t("search.score"),
  };
  return sortOptionNames[props.sortBy];
});

function init() {
  loadingIndicator.value.setLoading();
  getNotes(props.searchTerm)
    .then((data) => {
      results.value = sortResults(data);
      if (results.value.length > 0) {
        loadingIndicator.value.setLoaded();
      } else {
        loadingIndicator.value.setFailed(t("search.noResults"), Search);
      }
    })
    .catch((error) => {
      loadingIndicator.value.setFailed();
      apiErrorHandler(error, toast);
    });
}

function sortResults(results) {
  if (props.sortBy === searchSortOptions.title) {
    return results.sort((a, b) => a.title.localeCompare(b.title));
  } else if (props.sortBy === searchSortOptions.lastModified) {
    return results.sort((a, b) => b.lastModified - a.lastModified);
  } else {
    return results.sort((a, b) => b.score - a.score);
  }
}

function resultListTitle(result) {
  if (result.title !== defaultNoteTitle) return result.titleHighlightsOrTitle;
  return result.preview || defaultNoteTitle;
}

function reSortResults() {
  results.value = sortResults(results.value);
}

function updateSortByParam(sortBy) {
  router.push({
    name: "search",
    query: {
      [params.searchTerm]: props.searchTerm,
      [params.sortBy]: sortBy,
    },
  });
}

const menuItems = computed(() => [
  {
    label: t("search.sortBy", { name: t("search.score") }),
    command: () => {
      updateSortByParam(searchSortOptions.score);
    },
  },

  {
    label: t("search.sortBy", { name: t("search.title") }),
    command: () => {
      updateSortByParam(searchSortOptions.title);
    },
  },
  {
    label: t("search.sortBy", { name: t("search.lastModified") }),
    command: () => {
      updateSortByParam(searchSortOptions.lastModified);
    },
  },
]);

watch(() => props.searchTerm, init);
watch(() => props.sortBy, reSortResults);
onMounted(init);
</script>

<style>
.match {
  @apply text-theme-brand;
}
</style>
