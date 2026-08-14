<template>
  <ConfirmModal
    v-model="isDeleteModalVisible"
    :title="t('note.deleteTitle')"
    :message="t('note.deleteMessage', { title: contextNote?.title || '' })"
    :confirmButtonText="t('common.delete')"
    confirmButtonStyle="danger"
    @confirm="deleteContextNote"
  />

  <Dialog v-model="isTagDialogVisible" class="max-w-sm">
    <div class="space-y-4">
      <div>
        <h2 class="text-lg font-semibold">{{ t("note.addTagTitle") }}</h2>
        <p class="mt-1 text-sm text-theme-text-muted">
          {{ t("note.addTagHint") }}
        </p>
      </div>
      <Input
        v-model="tagValue"
        :placeholder="t('note.tagPlaceholder')"
        @keydown.enter="addTag"
      />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" @click="isTagDialogVisible = false">{{
          t("common.cancel")
        }}</Button>
        <Button :disabled="!tagValue.trim()" @click="addTag">{{
          t("common.confirm")
        }}</Button>
      </div>
    </div>
  </Dialog>

  <div v-if="grouped" class="min-h-0 flex-1 overflow-y-auto px-4 pb-6 pt-3">
    <section
      v-for="group in groupedNotes"
      :key="group.id"
      class="mb-7 last:mb-0"
    >
      <h2 class="mb-3 px-1 text-sm font-medium text-theme-text-muted">
        {{ t(`sidebar.timeGroups.${group.id}`) }}
      </h2>
      <div class="space-y-2.5">
        <RouterLink
          v-for="note in group.notes"
          :key="note.title"
          :to="{ name: 'note', params: { title: note.title } }"
          class="block rounded-xl border border-transparent bg-theme-background px-4 py-3.5 outline-none transition-colors focus-visible:ring-2 focus-visible:ring-theme-brand/60"
          :class="groupedNoteClasses(note)"
          @click="navigateNote"
          @contextmenu.prevent="openContextMenu(note, $event)"
          @pointerdown="startLongPress(note, $event)"
          @pointerup="cancelLongPress"
          @pointercancel="cancelLongPress"
          @pointermove="cancelLongPress"
        >
          <p class="line-clamp-2 text-[17px] leading-6 text-theme-text">
            {{ noteListTitle(note) }}
          </p>
          <p
            v-if="notePreview(note)"
            class="mt-1 line-clamp-1 text-sm leading-5 text-theme-text-muted"
          >
            {{ notePreview(note) }}
          </p>
        </RouterLink>
      </div>
    </section>

    <p
      v-if="loaded && notes.length === 0"
      class="px-1 pt-4 text-sm text-theme-text-very-muted"
    >
      {{ t("sidebar.noNotes") }}
    </p>
  </div>

  <div v-else class="min-h-0 flex-1 overflow-y-auto px-4 pb-5">
    <RouterLink
      v-for="note in notes"
      :key="note.title"
      :to="{ name: 'note', params: { title: note.title } }"
      class="mb-0.5 block border border-transparent px-5 py-3 outline-none transition-colors focus-visible:ring-2 focus-visible:ring-theme-brand/60"
      :class="noteClasses(note)"
      @click="navigateNote"
      @contextmenu.prevent="openContextMenu(note, $event)"
      @pointerdown="startLongPress(note, $event)"
      @pointerup="cancelLongPress"
      @pointercancel="cancelLongPress"
      @pointermove="cancelLongPress"
    >
      <div class="flex min-w-0 items-center gap-1.5">
        <div
          class="min-w-0 flex-1 truncate text-base font-semibold leading-5 text-theme-text"
        >
          {{ noteListTitle(note) }}
        </div>
        <Pin
          v-if="isPinned(note)"
          class="h-3.5 w-3.5 shrink-0 text-theme-brand-strong"
        />
        <LockKeyhole
          v-if="isLocked(note)"
          class="h-3.5 w-3.5 shrink-0 text-theme-text-very-muted"
        />
      </div>
      <p
        v-if="note.title !== defaultNoteTitle && notePreview(note)"
        class="leading-4.5 mt-0.5 block w-full truncate text-sm text-theme-text-muted"
        :title="notePreview(note)"
      >
        {{ notePreview(note) }}
      </p>
      <div
        class="mt-1.5 inline-flex rounded bg-theme-background-elevated px-1.5 py-0.5 text-[11px] leading-none text-theme-text-very-muted"
      >
        {{ displayDate(note.lastModified) }}
      </div>
    </RouterLink>
    <p
      v-if="loaded && notes.length === 0"
      class="px-4 pt-4 text-sm text-theme-text-very-muted"
    >
      {{ t("sidebar.noNotes") }}
    </p>
  </div>

  <NoteContextMenu
    v-model="isContextMenuVisible"
    compact
    :position="contextPosition"
    :items="contextMenuItems"
    @select="handleContextAction"
  />
</template>

<script setup lang="ts">
import {
  AlertCircle,
  LockKeyhole,
  LockKeyholeOpen,
  Pin,
  PinOff,
  Tag,
  Trash2,
} from "lucide-vue-next";
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { RouterLink, useRoute, useRouter } from "vue-router";

import {
  apiErrorHandler,
  deleteNote,
  getNote,
  getNotes,
  updateNote,
} from "../../api";
import ConfirmModal from "../common/ConfirmModal.vue";
import NoteContextMenu from "./NoteContextMenu.vue";
import Button from "../ui/Button.vue";
import Dialog from "../ui/Dialog.vue";
import Input from "../ui/Input.vue";
import { useToast } from "../../composables/useToast";
import { defaultNoteTitle } from "../../utils/constants";
import { useGlobalStore } from "../../stores/globalStore";
import { getNoteMetadata, setNoteMetadata } from "../../utils/noteMetadata";
import type { SearchResult } from "../../types/classes";

const props = withDefaults(
  defineProps<{
    grouped?: boolean;
  }>(),
  {
    grouped: false,
  },
);
const emit = defineEmits(["navigate"]);
const { locale, t } = useI18n();
const globalStore = useGlobalStore();
const loaded = ref(false);
const notes = ref<SearchResult[]>([]);
const route = useRoute();
const router = useRouter();
const toast = useToast();
const contextNote = ref<SearchResult | null>(null);
const contextPosition = ref({ x: 16, y: 16 });
const isContextMenuVisible = ref(false);
const isDeleteModalVisible = ref(false);
const isTagDialogVisible = ref(false);
const tagValue = ref("");
let longPressTimer: number | undefined;
let longPressTriggered = false;
let noteLoadRequest = 0;

type TimeGroup = "today" | "yesterday" | "week" | "earlier";
interface GroupedNotes {
  id: TimeGroup;
  notes: SearchResult[];
}

const groupedNotes = computed<GroupedNotes[]>(() => {
  const groups: Record<TimeGroup, SearchResult[]> = {
    today: [],
    yesterday: [],
    week: [],
    earlier: [],
  };

  for (const note of notes.value)
    groups[timeGroup(note.lastModified)].push(note);

  return (Object.keys(groups) as TimeGroup[])
    .map((id) => ({ id, notes: groups[id] }))
    .filter((group) => group.notes.length);
});

const contextMenuItems = computed(() => {
  const selected = contextNote.value;
  if (!selected) return [];
  const importance = getNoteMetadata(
    globalStore.currentUser?.id,
    selected.title,
    "importance",
    "none",
  );
  return [
    {
      id: "pin",
      label: isPinned(selected) ? t("note.unpin") : t("note.pin"),
      icon: isPinned(selected) ? PinOff : Pin,
    },
    {
      id: "lock",
      label: isLocked(selected) ? t("note.unlock") : t("note.lock"),
      icon: isLocked(selected) ? LockKeyholeOpen : LockKeyhole,
    },
    { separator: true },
    {
      id: "importance",
      label: t("note.setImportance"),
      icon: AlertCircle,
      children: [
        {
          id: "importance:none",
          label: t("note.importanceNone"),
          active: importance === "none",
        },
        {
          id: "importance:low",
          label: t("note.importanceLow"),
          active: importance === "low",
        },
        {
          id: "importance:medium",
          label: t("note.importanceMedium"),
          active: importance === "medium",
        },
        {
          id: "importance:high",
          label: t("note.importanceHigh"),
          active: importance === "high",
        },
      ],
    },
    { id: "tag", label: t("note.addTag"), icon: Tag },
    { separator: true },
    { id: "delete", label: t("common.delete"), icon: Trash2, danger: true },
  ];
});

async function loadNotes() {
  const request = ++noteLoadRequest;
  try {
    const loadedNotes = await getNotes(
      "*",
      "lastModified",
      "desc",
      props.grouped ? undefined : 80,
    );
    if (request !== noteLoadRequest) return;

    notes.value = props.grouped
      ? sortByLastModified(loadedNotes)
      : sortNotes(loadedNotes);
    hydrateMissingPreviews(loadedNotes, request);
  } catch (error) {
    apiErrorHandler(error, toast);
  } finally {
    loaded.value = true;
  }
}

async function hydrateMissingPreviews(items: SearchResult[], request: number) {
  const missing = items.filter((note) => !notePreview(note));
  if (!missing.length) return;

  const previews = await Promise.all(
    missing.map(async (note): Promise<[string, string]> => {
      try {
        const fullNote = await getNote(note.title);
        return [note.title, previewFromContent(fullNote.content)];
      } catch {
        return [note.title, ""];
      }
    }),
  );

  if (request !== noteLoadRequest) return;
  const previewByTitle = new Map(previews);
  notes.value.forEach((note) => {
    const preview = previewByTitle.get(note.title);
    if (preview) note.preview = preview;
  });
  notes.value = [...notes.value];
}

function displayDate(timestamp) {
  const date = new Date(timestamp * 1000);
  const currentYear = new Date().getFullYear();

  if (date.getFullYear() === currentYear) {
    return locale.value === "zh-CN"
      ? `${date.getMonth() + 1}月${date.getDate()}日`
      : date.toLocaleDateString(locale.value, {
          month: "short",
          day: "numeric",
        });
  }

  return locale.value === "zh-CN"
    ? `${date.getFullYear()}/${date.getMonth() + 1}/${date.getDate()}`
    : date.toLocaleDateString(locale.value, {
        year: "numeric",
        month: "numeric",
        day: "numeric",
      });
}

function notePreview(note) {
  return previewFromContent(note.preview || note.content);
}

function noteListTitle(note) {
  if (note.title !== defaultNoteTitle) return note.title;
  return notePreview(note) || defaultNoteTitle;
}

function previewFromContent(content) {
  return String(content || "")
    .replace(/<[^>]*>/g, " ")
    .replace(/\*\*|__|~~|`/g, "")
    .replace(/^\s{0,3}#{1,6}\s+/gm, "")
    .replace(/\s+/g, " ")
    .trim();
}

function groupedNoteClasses(note: SearchResult) {
  return (route.name === "note" && route.params.title === note.title) ||
    (isContextMenuVisible.value && contextNote.value?.title === note.title)
    ? "border-theme-brand/70 bg-theme-brand-soft"
    : "hover:border-theme-border hover:bg-theme-background-elevated";
}

function noteClasses(note) {
  return (route.name === "note" && route.params.title === note.title) ||
    (isContextMenuVisible.value && contextNote.value?.title === note.title)
    ? "rounded-xl bg-theme-sidebar-active"
    : "border-b-theme-border hover:rounded-xl hover:bg-theme-sidebar-active";
}

function isPinned(note) {
  return (
    getNoteMetadata(globalStore.currentUser?.id, note.title, "pinned") ===
    "true"
  );
}

function isLocked(note) {
  return (
    getNoteMetadata(globalStore.currentUser?.id, note.title, "locked") ===
    "true"
  );
}

function sortNotes(items: SearchResult[] = notes.value) {
  return [...items].sort((left, right) => {
    const pinnedDifference = Number(isPinned(right)) - Number(isPinned(left));
    return pinnedDifference || right.lastModified - left.lastModified;
  });
}

function sortByLastModified(items: SearchResult[]) {
  return [...items].sort(
    (left, right) => right.lastModified - left.lastModified,
  );
}

function timeGroup(timestamp: number): TimeGroup {
  const today = startOfDay(new Date());
  const noteDay = startOfDay(new Date(timestamp * 1000));
  const daysAgo = Math.floor(
    (today.getTime() - noteDay.getTime()) / 86_400_000,
  );

  if (daysAgo <= 0) return "today";
  if (daysAgo === 1) return "yesterday";
  if (daysAgo < 7) return "week";
  return "earlier";
}

function startOfDay(date: Date) {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate());
}

function openContextMenu(note, event) {
  cancelLongPress();
  contextNote.value = note;
  contextPosition.value = { x: event.clientX, y: event.clientY };
  isContextMenuVisible.value = true;
}

function startLongPress(note, event) {
  if (event.pointerType !== "touch") return;
  cancelLongPress();
  longPressTimer = window.setTimeout(() => {
    longPressTriggered = true;
    contextNote.value = note;
    contextPosition.value = { x: event.clientX, y: event.clientY };
    isContextMenuVisible.value = true;
    navigator.vibrate?.(12);
  }, 520);
}

function cancelLongPress() {
  if (longPressTimer) window.clearTimeout(longPressTimer);
  longPressTimer = undefined;
}

function navigateNote(event) {
  if (longPressTriggered) {
    event.preventDefault();
    longPressTriggered = false;
    return;
  }
  emit("navigate");
}

function handleContextAction(action) {
  const selected = contextNote.value;
  if (!selected) return;

  if (action === "pin") {
    setNoteMetadata(
      globalStore.currentUser?.id,
      selected.title,
      "pinned",
      !isPinned(selected),
    );
    notes.value = sortNotes();
    contextNote.value = { ...selected };
  } else if (action === "lock") {
    setNoteMetadata(
      globalStore.currentUser?.id,
      selected.title,
      "locked",
      !isLocked(selected),
    );
    notes.value = [...notes.value];
    contextNote.value = { ...selected };
  } else if (action.startsWith("importance:")) {
    setNoteMetadata(
      globalStore.currentUser?.id,
      selected.title,
      "importance",
      action.split(":")[1],
    );
    contextNote.value = { ...selected };
  } else if (action === "tag") {
    tagValue.value = "";
    isTagDialogVisible.value = true;
  } else if (action === "delete") {
    isDeleteModalVisible.value = true;
  }
}

async function addTag() {
  const selected = contextNote.value;
  const tag = tagValue.value.trim().replace(/^#/, "").replace(/\s+/g, "");
  if (!selected || !tag) return;
  try {
    const note = await getNote(selected.title);
    const content = note.content || "";
    const tagText = `#${tag}`;
    if (!content.includes(tagText)) {
      await updateNote(
        note.title,
        note.title,
        `${content.trimEnd()}${content.trim() ? "\n\n" : ""}${tagText}`,
      );
    }
    isTagDialogVisible.value = false;
    toast.add({ summary: t("note.tagAdded"), severity: "success" });
    await loadNotes();
  } catch (error) {
    apiErrorHandler(error, toast);
  }
}

async function deleteContextNote() {
  if (!contextNote.value) return;
  try {
    await deleteNote(contextNote.value.title);
    if (
      route.name === "note" &&
      route.params.title === contextNote.value.title
    ) {
      router.push({ name: "home" });
    }
    await loadNotes();
  } catch (error) {
    apiErrorHandler(error, toast);
  }
}

watch(() => route.fullPath, loadNotes);
watch(() => globalStore.currentUser?.id, loadNotes);
onMounted(() => {
  window.addEventListener("mionote:notes-changed", loadNotes);
  loadNotes();
});
onBeforeUnmount(() => {
  window.removeEventListener("mionote:notes-changed", loadNotes);
  cancelLongPress();
});
</script>
