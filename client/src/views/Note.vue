<template>
  <!-- Confirm Deletion Modal -->
  <ConfirmModal
    v-model="isDeleteModalVisible"
    :title="t('note.deleteTitle')"
    :message="t('note.deleteMessage', { title: displayNoteTitle(note.title) })"
    :confirmButtonText="t('common.delete')"
    confirmButtonStyle="danger"
    @confirm="deleteConfirmedHandler"
  />

  <!-- Save Changes Modal -->
  <ConfirmModal
    v-model="isSaveChangesModalVisible"
    :title="t('note.saveChangesTitle')"
    :message="t('note.saveChangesMessage')"
    :confirmButtonText="t('common.save')"
    confirmButtonStyle="success"
    :rejectButtonText="t('common.discard')"
    rejectButtonStyle="danger"
    @confirm="savePendingCloseAction"
    @reject="discardPendingCloseAction"
  />

  <!-- Draft Modal -->
  <ConfirmModal
    v-model="isDraftModalVisible"
    :title="t('note.draftTitle')"
    :message="t('note.draftMessage')"
    :confirmButtonText="t('note.resumeDraft')"
    confirmButtonStyle="cta"
    :rejectButtonText="t('note.deleteDraft')"
    rejectButtonStyle="danger"
    @confirm="setEditMode()"
    @reject="
      clearDraft();
      setEditMode();
    "
  />

  <Dialog v-model="isTitleDialogVisible" class="max-w-sm">
    <form class="space-y-4" @submit.prevent="saveTitle">
      <h2 class="text-lg font-semibold">{{ t("note.setTitle") }}</h2>
      <Input v-model="titleInput" :placeholder="t('note.title')" autofocus />
      <div class="flex justify-end gap-2">
        <Button
          type="button"
          variant="ghost"
          @click="isTitleDialogVisible = false"
        >
          {{ t("common.cancel") }}
        </Button>
        <Button type="submit" :disabled="!titleInput.trim()">
          {{ t("common.confirm") }}
        </Button>
      </div>
    </form>
  </Dialog>

  <Dialog v-model="isReminderDialogVisible" class="max-w-md">
    <div class="space-y-4">
      <div>
        <h2 class="text-lg font-semibold">{{ t("note.addReminder") }}</h2>
        <p class="mt-1 text-sm text-theme-text-muted">
          {{ t("note.reminderHint") }}
        </p>
      </div>
      <Input v-model="reminderAt" type="datetime-local" />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" @click="isReminderDialogVisible = false">{{
          t("common.cancel")
        }}</Button>
        <Button :disabled="!reminderAt" @click="saveReminder">{{
          t("common.confirm")
        }}</Button>
      </div>
    </div>
  </Dialog>

  <Dialog v-model="isHistoryDialogVisible" class="max-w-xl">
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-lg font-semibold">{{ t("note.history") }}</h2>
        <span class="text-sm text-theme-text-muted">{{
          historyEntries.length
        }}</span>
      </div>
      <div
        v-if="historyEntries.length"
        class="max-h-[50vh] divide-y divide-theme-border overflow-y-auto rounded-lg border border-theme-border"
      >
        <button
          v-for="entry in historyEntries"
          :key="entry.savedAt"
          type="button"
          class="flex w-full items-center justify-between gap-4 px-4 py-3 text-left transition-colors hover:bg-theme-background-elevated"
          @click="restoreHistory(entry)"
        >
          <span class="min-w-0">
            <span class="block truncate text-sm font-medium">{{
              displayNoteTitle(entry.title) || t("note.titlePlaceholder")
            }}</span>
            <span class="mt-0.5 block text-xs text-theme-text-muted">{{
              formatSavedAt(entry.savedAt)
            }}</span>
          </span>
          <History class="h-4 w-4 shrink-0 text-theme-text-muted" />
        </button>
      </div>
      <p v-else class="py-6 text-center text-sm text-theme-text-muted">
        {{ t("note.noHistory") }}
      </p>
    </div>
  </Dialog>

  <Dialog v-model="isInfoDialogVisible" class="max-w-md">
    <div class="space-y-4">
      <h2 class="text-lg font-semibold">{{ t("note.info") }}</h2>
      <dl class="space-y-3 text-sm">
        <div class="flex items-center justify-between gap-5">
          <dt class="text-theme-text-muted">{{ t("note.characters") }}</dt>
          <dd class="font-medium">{{ editorCharacterCount }}</dd>
        </div>
        <div class="flex items-center justify-between gap-5">
          <dt class="text-theme-text-muted">{{ t("search.lastModified") }}</dt>
          <dd class="font-medium">
            {{
              note.lastModified ? formatSavedAt(note.lastModified * 1000) : "-"
            }}
          </dd>
        </div>
      </dl>
    </div>
  </Dialog>

  <LoadingIndicator
    ref="loadingIndicator"
    class="flex h-full w-full flex-col"
  >
    <section
      ref="workspaceElement"
      class="flex min-h-0 flex-1 flex-col bg-theme-canvas pt-[max(0.75rem,env(safe-area-inset-top))] md:pt-0"
      :style="editorStyle"
    >
      <div
        class="relative flex h-12 shrink-0 items-center justify-between border-b border-theme-border md:h-14 md:pl-[var(--note-page-margin,1.5rem)] print:hidden"
      >
        <div class="flex items-center gap-1">
          <Button
            variant="ghost"
            size="icon-sm"
            class="bg-theme-background-elevated text-theme-text ring-1 ring-theme-border/70 hover:bg-theme-sidebar-active hover:text-theme-text hover:shadow-sm focus-visible:shadow-sm md:hidden"
            :aria-label="t('nav.allNotes')"
            @click="returnToNotes"
          >
            <ArrowLeft class="h-5 w-5 -translate-x-0.5" />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            class="hidden md:inline-flex"
            :title="t('nav.fullscreen')"
            :aria-label="t('nav.fullscreen')"
            @click="toggleFullscreen"
          >
            <Fullscreen class="h-5 w-5" />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            class="max-md:hidden md:h-10 md:w-10"
            :title="t('nav.undo')"
            :aria-label="t('nav.undo')"
            :disabled="!editMode"
            @click="runEditorCommand('undo')"
          >
            <Undo2 class="h-4 w-4 md:h-5 md:w-5" />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            class="max-md:hidden md:h-10 md:w-10"
            :title="t('nav.redo')"
            :aria-label="t('nav.redo')"
            :disabled="!editMode"
            @click="runEditorCommand('redo')"
          >
            <Redo2 class="h-4 w-4 md:h-5 md:w-5" />
          </Button>
        </div>

        <div class="flex shrink-0 items-center gap-0.5 pr-1 md:gap-1">
          <Button
            variant="ghost"
            size="icon-sm"
            class="hidden md:inline-flex md:h-10 md:w-10"
            :class="
              isFindPanelVisible
                ? 'bg-theme-background-elevated text-theme-text'
                : ''
            "
            :title="t('note.find')"
            :aria-label="t('note.find')"
            @click="togglePanel('find')"
          >
            <FileSearch class="h-4 w-4 md:h-5 md:w-5" />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            class="hidden md:inline-flex md:h-10 md:w-10"
            :class="
              isStylePanelVisible
                ? 'bg-theme-background-elevated text-theme-text'
                : ''
            "
            :title="t('note.paperStyle')"
            :aria-label="t('note.paperStyle')"
            @click="togglePanel('style')"
          >
            <SlidersHorizontal class="h-4 w-4 md:h-5 md:w-5" />
          </Button>
          <NoteMoreMenu :items="moreItems">
            <template #trigger>
              <Button
                variant="ghost"
                size="icon-sm"
                class="md:h-10 md:w-10"
                :aria-label="t('nav.menu')"
              >
                <MoreVertical class="h-4 w-4 md:h-5 md:w-5" />
              </Button>
            </template>
          </NoteMoreMenu>
        </div>
        <Transition name="mionote-panel">
          <NoteFindPanel
            v-if="isFindPanelVisible"
            :match-count="findMatchCount"
            @close="isFindPanelVisible = false"
            @find="findInNote"
            @replace="replaceNext"
            @replace-all="replaceAll"
          />
        </Transition>
        <Transition name="mionote-panel">
          <NoteStylePanel
            v-if="isStylePanelVisible"
            :paper="paperStyle"
            :paper-texture="paperTexture"
            :page-margin="pageMargin"
            @close="isStylePanelVisible = false"
            @update:paper="updatePaperStyle"
            @update:paper-texture="updatePaperTexture"
            @update:page-margin="updatePageMargin"
          />
        </Transition>
      </div>

      <div
        v-if="!editMode"
        class="min-h-0 flex-1 overflow-y-auto"
        @contextmenu.capture="openNoteContextMenu"
      >
        <div class="note-page-padding pb-8">
          <h1
            v-if="note.title"
            class="mb-4 text-2xl font-bold text-theme-text sm:text-3xl"
          >
            {{ displayNoteTitle(note.title) }}
          </h1>
          <RichTextViewer :html="note.content" />
        </div>
      </div>

      <div v-else class="flex min-h-0 flex-1 flex-col">
        <EditorToolbar :editor="toastEditor" />
        <div
          class="note-page-scroll min-h-0 flex-1 overflow-y-auto"
          @contextmenu.capture="openNoteContextMenu"
        >
          <div class="note-page-padding">
            <input
              v-model="noteTitleField"
              type="text"
              class="w-full bg-transparent text-2xl font-bold text-theme-text outline-none placeholder:text-theme-text-very-muted sm:text-3xl"
              :placeholder="t('note.title')"
              @keydown.enter.prevent="focusEditor"
            />
            <div class="mt-1.5 text-xs text-theme-text-muted">
              {{ t("note.characterCount", { count: editorCharacterCount }) }}
            </div>
            <TiptapEditor
              ref="toastEditor"
              class="mt-4"
              :initialValue="getInitialEditorValue()"
              :addImageBlobHook="addImageBlobHook"
              @change="editorChanged"
              @keydown="keydownHandler"
              @ready="captureBaseline"
            />
          </div>
        </div>
      </div>
    </section>
  </LoadingIndicator>
  <NoteContextMenu
    v-model="isContextMenuVisible"
    layout="horizontal"
    :position="contextPosition"
    :items="editorContextItems"
    @select="handleEditorContextAction"
  />
</template>

<style>
.note-page-padding {
  padding-inline: var(--note-page-margin, 1.5rem);
  padding-top: 1.25rem;
}

.note-page-scroll {
  /* Keep the last lines clear of the fixed mobile formatting bar. */
  scroll-padding-bottom: 4rem;
}

@media (max-width: 767px) {
  .note-page-padding {
    padding-bottom: calc(4rem + env(safe-area-inset-bottom));
  }
}
</style>

<script setup lang="ts">
import {
  ArrowLeft,
  BellRing,
  ClipboardPaste,
  Copy,
  Download,
  FileImage,
  FilePenLine,
  FileText,
  FileSearch,
  FileWarning,
  Fullscreen,
  History,
  Info,
  LockKeyhole,
  LockKeyholeOpen,
  ListChecks,
  MoreVertical,
  FileType2,
  FileDown,
  Redo2,
  Scissors,
  Search,
  SlidersHorizontal,
  Trash2,
  Undo2,
} from "lucide-vue-next";
import Mousetrap from "mousetrap";
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import {
  apiErrorHandler,
  createAttachment,
  createNote,
  deleteNote,
  getNote,
  updateNote,
} from "../api";
import { Note } from "../types/classes";
import ConfirmModal from "../components/common/ConfirmModal.vue";
import LoadingIndicator from "../components/common/LoadingIndicator.vue";
import TiptapEditor from "../components/editor/TiptapEditor.vue";
import EditorToolbar from "../components/editor/EditorToolbar.vue";
import RichTextViewer from "../components/editor/RichTextViewer.vue";
import Button from "../components/ui/Button.vue";
import Dialog from "../components/ui/Dialog.vue";
import Input from "../components/ui/Input.vue";
import { useToast } from "../composables/useToast";
import {
  defaultNoteTitle,
  isUntitledNoteTitle,
} from "../utils/constants";
import { useGlobalStore } from "../stores/globalStore";
import { getToastOptions } from "../utils/helpers";
import { isCurrentTokenStored } from "../api/tokenStorage";
import NoteFindPanel from "../components/notes/NoteFindPanel.vue";
import NoteContextMenu from "../components/notes/NoteContextMenu.vue";
import NoteMoreMenu from "../components/notes/NoteMoreMenu.vue";
import NoteStylePanel from "../components/notes/NoteStylePanel.vue";
import {
  getNoteMetadata,
  removeNoteMetadata,
  setNoteMetadata,
} from "../utils/noteMetadata";
import type { NoteHistoryEntry, NoteEditorHandle } from "../types";

const props = defineProps<{ title?: string }>();

type CloseAction = "edit" | "notes";

const emit = defineEmits<{ (event: "open-search"): void }>();

const { locale, t } = useI18n();
const canModify = computed(() => true);
let contentChangedTimeout: number | null = null;
let autoSaveInProgress = false;
let autoSaveQueued = false;
// Last-known persisted HTML. Tiptap re-normalizes on load, so comparing against
// `note.value.content` flaps; compare against this baseline instead.
let baselineHtml = "";
const editMode = ref(false);
const editorCharacterCount = ref(0);
const historyEntries = ref<NoteHistoryEntry[]>([]);
const globalStore = useGlobalStore();
const isFindPanelVisible = ref(false);
const findMatchCount = ref({ current: 0, total: 0 });
let activeFindKey = "";
let activeFindIndex = -1;
const isHistoryDialogVisible = ref(false);
const isSaveChangesModalVisible = ref(false);
const pendingCloseAction = ref<CloseAction | null>(null);
const isDeleteModalVisible = ref(false);
const isDraftModalVisible = ref(false);
const isInfoDialogVisible = ref(false);
const isTitleDialogVisible = ref(false);
const isLocked = ref(false);
const isReminderDialogVisible = ref(false);
const isStylePanelVisible = ref(false);
const isContextMenuVisible = ref(false);
const isNewNote = computed(() => !note.value.title);
const loadingIndicator = ref<InstanceType<typeof LoadingIndicator> | null>(
  null,
);
const note = ref(new Note());
const paperColors = {
  // The default paper follows the active theme instead of forcing a white
  // editor surface in dark mode. Other paper choices remain explicit colors.
  plain: "rgb(var(--theme-canvas))",
  mist: "#f5f5f4",
  cream: "#fffde8",
  peach: "#fff2e9",
  sky: "#edf7ff",
};
const paperStyle = ref(localStorage.getItem("mionote:paper-style") || "plain");
const paperTexture = ref(
  Number(localStorage.getItem("mionote:paper-texture")) || 0,
);
const pageMargin = ref(
  Number(localStorage.getItem("mionote:page-margin")) || 24,
);
const reservedFilenameCharacters = /[<>:"/\\|?*]/;
const reminderAt = ref("");
const router = useRouter();
const newTitle = ref(defaultNoteTitle);
const titleInput = ref("");
const toast = useToast();
const toastEditor = ref<NoteEditorHandle | null>(null);
const unsavedChanges = ref(false);
const workspaceElement = ref<HTMLElement | null>(null);
const editorStyle = computed(() => ({
  "--note-paper-color": paperColors[paperStyle.value] || paperColors.plain,
  "--note-paper-shadow-alpha": `${paperTexture.value / 900}`,
  "--note-page-margin": `${pageMargin.value}px`,
}));
const contextPosition = ref({ x: 16, y: 16 });
const selectedText = ref("");
const editorContextItems = computed(() => {
  const editable = editMode.value && !isLocked.value;
  return [
    { id: "copy", label: t("editor.copy"), icon: Copy },
    { id: "cut", label: t("editor.cut"), icon: Scissors, disabled: !editable },
    {
      id: "paste",
      label: t("editor.paste"),
      icon: ClipboardPaste,
      disabled: !editable,
    },
    {
      id: "delete-selection",
      label: t("editor.deleteSelection"),
      icon: Trash2,
      danger: true,
      disabled: !editable,
    },
    { id: "select-all", label: t("editor.selectAll"), icon: ListChecks },
  ];
});
const moreItems = computed(() => [
  {
    label: t("nav.search"),
    icon: Search,
    mobileOnly: true,
    command: () => emit("open-search"),
  },
  {
    label: t("note.find"),
    icon: FileSearch,
    mobileOnly: true,
    command: () => togglePanel("find"),
  },
  {
    label: t("note.paperStyle"),
    icon: SlidersHorizontal,
    mobileOnly: true,
    command: () => togglePanel("style"),
  },
  { separator: true, mobileOnly: true },
  {
    label: t("note.setTitle"),
    icon: FilePenLine,
    command: openTitleDialog,
  },
  { label: t("note.addReminder"), icon: BellRing, command: openReminder },
  {
    label: t("common.delete"),
    icon: Trash2,
    danger: true,
    disabled: isNewNote.value,
    command: deleteHandler,
  },
  {
    label: isLocked.value ? t("note.unlock") : t("note.lock"),
    icon: isLocked.value ? LockKeyholeOpen : LockKeyhole,
    command: toggleLock,
  },
  {
    label: t("note.export"),
    icon: Download,
    children: [
      {
        label: t("note.exportImage"),
        icon: FileImage,
        command: () => exportNote("image"),
      },
      {
        label: t("note.exportWord"),
        icon: FileType2,
        command: () => exportNote("word"),
      },
      {
        label: t("note.exportText"),
        icon: FileText,
        command: () => exportNote("text"),
      },
      {
        label: t("note.exportPdf"),
        icon: FileDown,
        command: () => exportNote("pdf"),
      },
    ],
  },
  { separator: true },
  { label: t("note.history"), icon: History, command: openHistory },
  {
    label: t("note.info"),
    icon: Info,
    command: () => (isInfoDialogVisible.value = true),
  },
]);

function init() {
  // Return if we already have the note e.g. When we rename a note, the route prop would change but we’d already have the note.
  if (props.title && props.title == note.value.title) {
    return;
  }
  const currentContent =
    toastEditor.value?.getHTML() ?? note.value.content;
  if (
    !props.title &&
    isUntitledNoteTitle(note.value.title) &&
    editMode.value &&
    isEmptyHtml(currentContent)
  ) {
    router.replace({ name: "note", params: { title: note.value.title } });
    return;
  }

  loadingIndicator.value?.setLoading();
  if (props.title) {
    getNote(props.title)
      .then((data) => {
        note.value = data;
        isLocked.value = noteIsLocked(data.title);
        loadHistory();
        if (!isLocked.value) editHandler();
        loadingIndicator.value?.setLoaded();
      })
      .catch((error) => {
        if (error.response?.status === 404) {
          loadingIndicator.value?.setFailed(t("note.notFound"), FileWarning);
        } else {
          loadingIndicator.value?.setFailed();
          apiErrorHandler(error, toast);
        }
      });
  } else {
    openDefaultNote();
  }
}

function openDefaultNote() {
  const open = (data) => {
    note.value = data;
    isLocked.value = noteIsLocked(data.title);
    loadHistory();
    router.replace({ name: "note", params: { title: data.title } });
    if (!isLocked.value) editHandler();
    loadingIndicator.value?.setLoaded();
  };
  const fail = (error) => {
    loadingIndicator.value?.setFailed();
    apiErrorHandler(error, toast);
  };
  const createUntitledNote = () => {
    createNote(defaultNoteTitle, "")
      .then(open)
      .catch((createError) => {
        if (createError.response?.status === 409) {
          getNote(defaultNoteTitle).then(open).catch(fail);
        } else {
          fail(createError);
        }
      });
  };

  getNote(defaultNoteTitle)
    .then((data) => {
      if (!isEmptyHtml(data.content)) {
        createUntitledNote();
      } else {
        open(data);
      }
    })
    .catch((error) => {
      if (error.response?.status !== 404) {
        fail(error);
        return;
      }
      createUntitledNote();
    });
}

function runEditorCommand(command: string) {
  if (editMode.value) toastEditor.value?.exec(command);
}

async function toggleFullscreen() {
  if (document.fullscreenElement) {
    await document.exitFullscreen();
  } else {
    await workspaceElement.value?.requestFullscreen();
  }
}

function editHandler() {
  const draft = loadDraft();
  if (draft?.content || draft?.title) {
    isDraftModalVisible.value = true;
  } else {
    setEditMode();
  }
}

function setEditMode() {
  const draft = loadDraft();
  const draftTitle = typeof draft?.title === "string" ? draft.title : null;
  newTitle.value =
    draftTitle ?? editableTitleForStorageTitle(note.value.title);
  editorCharacterCount.value = countCharacters(
    draft?.content || note.value.content,
  );
  unsavedChanges.value = false;
  editMode.value = true;
  nextTick(captureBaseline);
}

function getInitialEditorValue() {
  const value = loadDraft()?.content || note.value.content;
  return isEmptyHtml(value) ? "" : value;
}

function editorChanged() {
  editorCharacterCount.value = countCharacters(
    toastEditor.value?.getHTML() || "",
  );
  startContentChangedTimeout();
}

function focusEditor() {
  toastEditor.value?.focus();
}

function captureBaseline() {
  baselineHtml = toastEditor.value?.getHTML() || "";
}

// The canonical new-note filename is a storage fallback, not visible title text.
const noteTitleField = computed({
  get: () =>
    isUntitledNoteTitle(note.value.title) &&
    (!newTitle.value || newTitle.value === defaultNoteTitle)
      ? ""
      : newTitle.value,
  set: (value: string) => {
    // Keep the field editable while the user is deleting the title. Trimming
    // is deferred until a title is persisted so an empty value is not written
    // back from the current note title.
    newTitle.value = value;
    startContentChangedTimeout();
  },
});

function persistedTitle() {
  const title = newTitle.value.trim();
  if (title) return title;
  return isUntitledNoteTitle(note.value.title)
    ? note.value.title
    : defaultNoteTitle;
}

function editableTitleForStorageTitle(title: string) {
  if (!title) return defaultNoteTitle;
  return isUntitledNoteTitle(title) && title !== defaultNoteTitle ? "" : title;
}

function displayNoteTitle(title: string) {
  return isUntitledNoteTitle(title) ? defaultNoteTitle : title;
}

function togglePanel(panel: "find" | "style") {
  if (panel === "find") {
    isFindPanelVisible.value = !isFindPanelVisible.value;
    isStylePanelVisible.value = false;
  } else {
    isStylePanelVisible.value = !isStylePanelVisible.value;
    isFindPanelVisible.value = false;
  }
}

function updatePaperStyle(style: string) {
  paperStyle.value = style;
  localStorage.setItem("mionote:paper-style", style);
}

function updatePaperTexture(value: number) {
  paperTexture.value = value;
  localStorage.setItem("mionote:paper-texture", String(value));
}

function updatePageMargin(value: number) {
  pageMargin.value = value;
  localStorage.setItem("mionote:page-margin", String(value));
}

function noteLocalKey(name: string, title = note.value.title || "new") {
  const userId = globalStore.currentUser?.id || "anonymous";
  return `mionote:note:${userId}:${title}:${name}`;
}

function noteIsLocked(title: string) {
  return (
    getNoteMetadata(globalStore.currentUser?.id, title, "locked") === "true"
  );
}

function toggleLock() {
  if (isLocked.value) {
    removeNoteMetadata(globalStore.currentUser?.id, note.value.title, "locked");
    isLocked.value = false;
    editHandler();
    return;
  }

  if (editMode.value && isContentChanged()) {
    saveDraft();
    autoSaveHandler();
  }
  setNoteMetadata(
    globalStore.currentUser?.id,
    note.value.title,
    "locked",
    true,
  );
  isLocked.value = true;
  editMode.value = false;
}

function openNoteContextMenu(event: MouseEvent & { pointerType?: string }) {
  // A touch long-press must remain browser-owned so the native selection
  // handles and its menu stay anchored to the selected text.
  const isTouch = event.pointerType === "touch";
  if (isTouch) return;

  event.preventDefault();
  event.stopPropagation();
  selectedText.value = window.getSelection?.()?.toString() || "";
  contextPosition.value = { x: event.clientX, y: event.clientY };
  isContextMenuVisible.value = true;
}

async function handleEditorContextAction(action: string) {
  const editor = toastEditor.value;
  if (action === "copy") {
    if (selectedText.value) await writeClipboard(selectedText.value);
  } else if (action === "cut") {
    if (!editor || isLocked.value || !editMode.value) return;
    if (selectedText.value) await writeClipboard(selectedText.value);
    editor.deleteSelection();
    startContentChangedTimeout();
  } else if (action === "paste") {
    if (!editor || isLocked.value || !editMode.value) return;
    const text = await readClipboard();
    if (text) {
      editor.insertText(text);
      startContentChangedTimeout();
    }
  } else if (action === "delete-selection") {
    if (!editor || isLocked.value || !editMode.value) return;
    editor.deleteSelection();
    startContentChangedTimeout();
  } else if (action === "select-all") {
    if (editor && editMode.value) {
      editor.exec("selectAll");
    } else {
      const selection = window.getSelection();
      const range = document.createRange();
      const content = workspaceElement.value?.querySelector(
        ".mionote-rich-text .tiptap",
      );
      if (content) {
        range.selectNodeContents(content);
        selection?.removeAllRanges();
        selection?.addRange(range);
      }
    }
  }
}

async function writeClipboard(text) {
  try {
    await navigator.clipboard?.writeText(text);
  } catch {
    // Clipboard access may be denied outside a secure browser context.
  }
}

async function readClipboard() {
  try {
    return (await navigator.clipboard?.readText()) || "";
  } catch {
    return "";
  }
}

function openReminder() {
  reminderAt.value = localStorage.getItem(noteLocalKey("reminder")) || "";
  isReminderDialogVisible.value = true;
}

function saveReminder() {
  localStorage.setItem(noteLocalKey("reminder"), reminderAt.value);
  isReminderDialogVisible.value = false;
  toast.add(
    getToastOptions(t("note.reminderSaved"), t("common.success"), "success"),
  );
}

function openTitleDialog() {
  titleInput.value =
    isUntitledNoteTitle(newTitle.value) ? "" : newTitle.value || "";
  isTitleDialogVisible.value = true;
}

function saveTitle() {
  const title = titleInput.value.trim();
  if (!title) return;
  if (reservedFilenameCharacters.test(title)) {
    badFilenameToast(t("note.title"));
    return;
  }
  newTitle.value = title;
  isTitleDialogVisible.value = false;
  startContentChangedTimeout();
}

function noteExportData() {
  const html = toastEditor.value?.getHTML() || note.value.content || "";
  const parser = document.createElement("div");
  parser.innerHTML = html;
  return {
    title:
      newTitle.value.trim() ||
      (isUntitledNoteTitle(note.value.title)
        ? defaultNoteTitle
        : note.value.title || "MioNote"),
    html,
    text: parser.innerText.trim(),
  };
}

function downloadBlob(blob, filename) {
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.append(link);
  link.click();
  link.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 1000);
}

function exportFilename(title) {
  return title.replace(reservedFilenameCharacters, "_").trim() || "MioNote";
}

function escapeExportHtml(value) {
  return String(value)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/\"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

function exportDocumentHtml({ title, html }) {
  return `<style>:scope{display:block;box-sizing:border-box;font-family:"PingFang SC","Microsoft YaHei",sans-serif;color:#1f2937;line-height:1.75;width:760px;margin:0;padding:48px;background:#fff}:scope h1{font-size:28px;margin:0 0 28px}:scope img,:scope video{max-width:100%;height:auto}:scope table{border-collapse:collapse;width:100%}:scope th,:scope td{border:1px solid #d1d5db;padding:8px;text-align:left}:scope pre{padding:12px;background:#f3f4f6;overflow:auto}</style><h1>${escapeExportHtml(title)}</h1>${html}`;
}

async function exportNote(format) {
  const data = noteExportData();
  const filename = exportFilename(data.title);

  if (format === "text") {
    downloadBlob(
      new Blob([data.text], { type: "text/plain;charset=utf-8" }),
      `${filename}.txt`,
    );
    return;
  }

  if (format === "word") {
    const { Document, Packer, Paragraph, TextRun } = await import("docx");
    const paragraphs = data.text
      .split(/\n+/)
      .map((line) => new Paragraph({ children: [new TextRun(line)] }));
    const document = new Document({
      sections: [
        {
          children: [
            new Paragraph({ text: data.title, heading: "Heading1" }),
            ...paragraphs,
          ],
        },
      ],
    });
    downloadBlob(await Packer.toBlob(document), `${filename}.docx`);
    return;
  }

  const exportSurface = document.createElement("article");
  exportSurface.className = "mionote-export-surface";
  exportSurface.style.cssText =
    "position: fixed; left: -10000px; top: 0; width: 856px; background: #fff; z-index: -1;";
  exportSurface.innerHTML = exportDocumentHtml(data);
  document.body.append(exportSurface);

  try {
    if (format === "image") {
      const { default: html2canvas } = await import("html2canvas");
      const canvas = await html2canvas(exportSurface, {
        backgroundColor: "#ffffff",
        scale: 2,
        useCORS: true,
      });
      const blob = await new Promise((resolve) =>
        canvas.toBlob(resolve, "image/png"),
      );
      if (blob) downloadBlob(blob, `${filename}.png`);
      return;
    }

    const [{ default: html2canvas }, { jsPDF }] = await Promise.all([
      import("html2canvas"),
      import("jspdf"),
    ]);
    const canvas = await html2canvas(exportSurface, {
      backgroundColor: "#ffffff",
      scale: 2,
      useCORS: true,
    });
    const pdf = new jsPDF({ orientation: "p", unit: "pt", format: "a4" });
    const pageWidth = pdf.internal.pageSize.getWidth();
    const pageHeight = pdf.internal.pageSize.getHeight();
    const imageHeight = (canvas.height * pageWidth) / canvas.width;
    const image = canvas.toDataURL("image/png");
    let remaining = imageHeight;
    let offset = 0;
    while (remaining > 0) {
      pdf.addImage(image, "PNG", 0, offset, pageWidth, imageHeight);
      remaining -= pageHeight;
      if (remaining > 0) {
        offset -= pageHeight;
        pdf.addPage();
      }
    }
    pdf.save(`${filename}.pdf`);
  } finally {
    exportSurface.remove();
  }
}

function loadHistory() {
  try {
    historyEntries.value = JSON.parse(
      localStorage.getItem(noteLocalKey("history")) || "[]",
    );
  } catch {
    historyEntries.value = [];
  }
}

function recordHistory(data) {
  if (!data?.title) return;

  const key = noteLocalKey("history", data.title);
  let entries = [];
  try {
    entries = JSON.parse(localStorage.getItem(key) || "[]");
  } catch {
    entries = [];
  }

  const current = entries[0];
  if (current?.title === data.title && current?.content === data.content)
    return;

  entries = [
    { title: data.title, content: data.content, savedAt: Date.now() },
    ...entries,
  ].slice(0, 20);
  localStorage.setItem(key, JSON.stringify(entries));
  historyEntries.value = entries;
}

function openHistory() {
  loadHistory();
  isHistoryDialogVisible.value = true;
}

function restoreHistory(entry) {
  const apply = () => {
    newTitle.value = editableTitleForStorageTitle(entry.title);
    toastEditor.value?.setHTML(entry.content);
    editorCharacterCount.value = countCharacters(entry.content);
    startContentChangedTimeout();
  };

  isHistoryDialogVisible.value = false;
  if (editMode.value) {
    apply();
  } else {
    setEditMode();
    nextTick(apply);
  }
}

function formatSavedAt(timestamp) {
  return new Date(timestamp).toLocaleString(locale.value, {
    dateStyle: "medium",
    timeStyle: "short",
  });
}

function findInNote({
  query,
  caseSensitive,
  backwards = false,
  reset = false,
  focusEditor = !reset,
}) {
  const match = getFindMatch(query, caseSensitive, backwards, reset);
  if (!match) {
    findMatchCount.value = { current: 0, total: 0 };
    toastEditor.value?.clearMatches();
    if (query)
      toast.add(
        getToastOptions(t("note.noMatches"), t("common.error"), "error"),
      );
    return;
  }

  findMatchCount.value = {
    current: activeFindIndex + 1,
    total: match.matches.length,
  };
  toastEditor.value?.highlightMatches(
    query,
    caseSensitive,
    match.matches[activeFindIndex],
  );
  if (focusEditor) {
    selectFindMatch(match.matches[activeFindIndex], query.length);
  }
}

function replaceNext(payload) {
  replaceInNote(payload, false);
}

function replaceAll(payload) {
  replaceInNote(payload, true);
}

function replaceInNote(
  { query, replacement, caseSensitive },
  replaceEveryMatch,
) {
  const editor = toastEditor.value;
  if (isLocked.value || !editor || !query) return;

  const text = editor.getVisibleText() || "";
  const source = caseSensitive ? text : text.toLocaleLowerCase();
  const needle = caseSensitive ? query : query.toLocaleLowerCase();
  const matches = findMatchOffsets(source, needle);
  if (!matches.length) {
    toast.add(getToastOptions(t("note.noMatches"), t("common.error"), "error"));
    return;
  }

  const findKey = `${caseSensitive}:${query}`;
  const targetIndex =
    activeFindKey === findKey && activeFindIndex >= 0 ? activeFindIndex : 0;
  const targets = replaceEveryMatch
    ? matches.slice()
    : [matches[targetIndex]];

  // Replace from the end so earlier offsets stay valid as the doc mutates.
  for (let index = targets.length - 1; index >= 0; index -= 1) {
    const offset = targets[index];
    if (offset == null) continue;
    editor.selectRange(offset, offset + query.length);
    editor.replaceSelection(replacement);
  }

  editorCharacterCount.value = countCharacters(editor.getHTML() || "");
  activeFindKey = "";
  activeFindIndex = -1;
  findInNote({ query, caseSensitive, reset: true });
  startContentChangedTimeout();
}

function getFindMatch(query, caseSensitive, backwards, reset) {
  if (!query) return null;
  const content = toastEditor.value?.getVisibleText() || "";
  const source = caseSensitive ? content : content.toLocaleLowerCase();
  const needle = caseSensitive ? query : query.toLocaleLowerCase();
  const matches = findMatchOffsets(source, needle);
  if (!matches.length) return null;

  const key = `${caseSensitive}:${query}`;
  if (reset || activeFindKey !== key) {
    activeFindKey = key;
    activeFindIndex = 0;
  } else if (backwards) {
    activeFindIndex = (activeFindIndex - 1 + matches.length) % matches.length;
  } else {
    activeFindIndex = (activeFindIndex + 1) % matches.length;
  }
  return { matches };
}

function findMatchOffsets(source, needle) {
  const matches = [];
  let offset = source.indexOf(needle);
  while (offset >= 0) {
    matches.push(offset);
    offset = source.indexOf(needle, offset + needle.length);
  }
  return matches;
}

function selectFindMatch(offset, length) {
  const editor = toastEditor.value;
  if (!editor || !editMode.value) return;
  // Select through the ProseMirror state so the selection survives editor
  // redraws; find offsets are visible-text offsets mapped by the editor.
  editor.selectRange(offset, offset + length);
}

// Note Deletion
function deleteHandler() {
  isDeleteModalVisible.value = true;
}

function deleteConfirmedHandler() {
  deleteNote(note.value.title)
    .then(() => {
      toast.add(
        getToastOptions(t("note.deleted"), t("common.success"), "success"),
      );
      router.push({ name: "home" });
    })
    .catch((error) => {
      apiErrorHandler(error, toast);
    });
}

// Note Saving
function saveHandler(closeAction: CloseAction | null = null) {
  const title = persistedTitle();

  // Invalid Character Validation
  if (reservedFilenameCharacters.test(title)) {
    badFilenameToast(t("note.title"));
    return;
  }

  if (!toastEditor.value) return;
  const newContent = toastEditor.value.getHTML();
  if (isNewNote.value) {
    saveNew(title, newContent, closeAction);
  } else {
    saveExisting(title, newContent, closeAction);
  }
}

function saveNew(newTitle, newContent, closeAction: CloseAction | null = null) {
  createNote(newTitle, newContent)
    .then((data) => {
      clearDraft();
      note.value = data;
      baselineHtml = data.content;
      recordHistory(data);
      notifyNotesChanged();
      router
        .push({
          name: "note",
          params: { title: note.value.title },
        })
        .then(() => {
          // Wait for the route to be updated before setting edit mode to false
          // as the route is used to determine the action.
          noteSaveSuccess(closeAction);
        });
    })
    .catch(noteSaveFailure);
}

function saveExisting(
  newTitle,
  newContent,
  closeAction: CloseAction | null = null,
) {
  // Return if no changes
  if (newTitle == note.value.title && newContent == baselineHtml) {
    noteSaveSuccess(closeAction);
    return;
  }

  updateNote(note.value.title, newTitle, newContent)
    .then((data) => {
      clearDraft();
      note.value = data;
      baselineHtml = data.content;
      recordHistory(data);
      notifyNotesChanged();
      router.replace({ name: "note", params: { title: note.value.title } });
      noteSaveSuccess(closeAction);
    })
    .catch(noteSaveFailure);
}

function noteSaveFailure(error) {
  if (error.response?.status === 409) {
    duplicateNoteToast(error);
  } else if (error.response?.status === 413) {
    entityTooLargeToast(t("note.title"));
  } else {
    apiErrorHandler(error, toast);
  }
}

function noteSaveSuccess(closeAction: CloseAction | null = null) {
  unsavedChanges.value = false;
  if (closeAction) {
    executeCloseAction(closeAction);
  }
  setBeforeUnloadConfirmation(false);
  toast.add(getToastOptions(t("note.saved"), t("common.success"), "success"));
}

function autoSaveHandler() {
  if (!editMode.value || !toastEditor.value) return;

  if (autoSaveInProgress) {
    autoSaveQueued = true;
    return;
  }
  const title = persistedTitle();
  const content = toastEditor.value.getHTML();
  if (title === note.value.title && content === baselineHtml) return;
  const isCreating = !note.value.title;
  const previousTitle = note.value.title;
  const draftKey = draftStorageKey();
  autoSaveInProgress = true;

  const request = isCreating
    ? createNote(title, content)
    : updateNote(previousTitle, title, content);

  request
    .then((data) => {
      note.value = data;
      baselineHtml = data.content;
      recordHistory(data);
      notifyNotesChanged();
      if (isCreating || previousTitle !== data.title) {
        router.replace({ name: "note", params: { title: data.title } });
      }

      if (
        persistedTitle() === data.title &&
        toastEditor.value?.getHTML() === data.content
      ) {
        unsavedChanges.value = false;
        setBeforeUnloadConfirmation(false);
        clearDraft(draftKey);
      } else {
        autoSaveQueued = true;
      }
    })
    .catch((error) => {
      if (error.response?.status === 409) {
        duplicateNoteToast(error);
      } else if (error.response?.status === 413) {
        entityTooLargeToast(t("note.title"));
      } else {
        apiErrorHandler(error, toast);
      }
    })
    .finally(() => {
      autoSaveInProgress = false;
      if (autoSaveQueued) {
        autoSaveQueued = false;
        autoSaveHandler();
      }
    });
}

function duplicateNoteToast(error) {
  const detail = String(error.response?.data?.detail || "");
  const isEmptyUntitledConflict =
    detail === "Cannot create a second untitled empty note." ||
    (!newTitle.value.trim() &&
      isEmptyHtml(toastEditor.value?.getHTML() || ""));
  const message =
    isEmptyUntitledConflict
      ? t("note.emptyUntitledDuplicate")
      : t("note.duplicate");
  toast.add(getToastOptions(message, t("common.error"), "error"));
}

// Note Closure
function closeHandler() {
  requestClose("edit");
}

function returnToNotes() {
  requestClose("notes");
}

function requestClose(action: CloseAction) {
  if (editMode.value && isContentChanged()) {
    pendingCloseAction.value = action;
    isSaveChangesModalVisible.value = true;
    return;
  }

  executeCloseAction(action);
}

function savePendingCloseAction() {
  saveHandler(pendingCloseAction.value ?? "edit");
}

function discardPendingCloseAction() {
  executeCloseAction(pendingCloseAction.value ?? "edit");
}

function executeCloseAction(action: CloseAction) {
  pendingCloseAction.value = null;
  clearDraft();
  editMode.value = false;
  if (action === "notes" || isNewNote.value) {
    router.push({ name: "home" });
  }
}

// Image Upload
function addImageBlobHook(
  file: File,
  callback: (url: string, altText?: string) => void,
) {
  // Upload the image then use the callback to insert the URL into the editor
  const request = postAttachment(file);
  if (!request) return Promise.reject(new Error("Invalid attachment"));
  return request.then(function (data) {
    if (!data) throw new Error("Attachment upload failed");
    // Use the filename returned by the API as the alt text.
    callback(data.url, data.filename);
    return data;
  });
}

function postAttachment(file: File) {
  // Invalid Character Validation
  if (reservedFilenameCharacters.test(file.name)) {
    badFilenameToast(t("note.attachment"));
    return;
  }

  // Uploading Toast
  toast.add(getToastOptions(t("note.uploading")));

  // Upload the attachment
  return createAttachment(file)
    .then((data) => {
      // Success Toast
      toast.add(
        getToastOptions(
          t("note.attachmentUploaded"),
          t("common.success"),
          "success",
        ),
      );
      return data;
    })
    .catch((error) => {
      if (error.response?.status === 409) {
        // Note: The current implementation will append a datetime to the filename if it already exists.
        // Error Toast
        toast.add(
          getToastOptions(t("note.duplicate"), t("common.error"), "error"),
        );
      } else if (error.response?.status == 413) {
        entityTooLargeToast(t("note.attachment"));
      } else {
        apiErrorHandler(error, toast);
      }
    });
}

// Content Change Watcher
function startContentChangedTimeout() {
  clearContentChangedTimeout();
  contentChangedTimeout = window.setTimeout(contentChangedHandler, 1000);
}

function clearContentChangedTimeout() {
  if (contentChangedTimeout != null) {
    clearTimeout(contentChangedTimeout);
  }
}

function contentChangedHandler() {
  if (!editMode.value || !toastEditor.value) return;

  if (isContentChanged()) {
    unsavedChanges.value = true;
    setBeforeUnloadConfirmation(true);
    saveDraft();
    autoSaveHandler();
  } else {
    unsavedChanges.value = false;
    setBeforeUnloadConfirmation(false);
    clearDraft();
  }
}

function isEmptyHtml(html: unknown) {
  return (
    String(html ?? "")
      .replace(/<[^>]*>/g, "")
      .replace(/&nbsp;/g, "")
      .trim() === ""
  );
}

function countCharacters(content: unknown) {
  // Content is stored as HTML; count visible characters only.
  return String(content ?? "")
    .replace(/<[^>]*>/g, "")
    .replace(/&nbsp;/g, "")
    .replace(/\s/g, "").length;
}

// Drafts
function saveDraft() {
  if (!toastEditor.value) return;
  const content = toastEditor.value.getHTML();
  const userHasPersistedToken = isCurrentTokenStored();
  const draft = JSON.stringify({ title: newTitle.value || "", content });
  if (content || newTitle.value) {
    if (userHasPersistedToken) {
      localStorage.setItem(draftStorageKey(), draft);
    } else {
      sessionStorage.setItem(draftStorageKey(), draft);
    }
  }
}

function clearDraft(key = draftStorageKey()) {
  localStorage.removeItem(key);
  sessionStorage.removeItem(key);
}

function loadDraft() {
  const localDraft = localStorage.getItem(draftStorageKey());
  const sessionDraft = sessionStorage.getItem(draftStorageKey());
  const value = localDraft || sessionDraft;
  if (!value) return null;

  try {
    const draft = JSON.parse(value);
    if (typeof draft?.content === "string") return draft;
  } catch {
    // Drafts created by earlier versions stored their content as a plain string.
  }

  return { title: "", content: value };
}

// Keyboard Shortcuts
// 'e' to edit
Mousetrap.bind("e", () => {
  if (editMode.value === false && canModify.value) {
    editHandler();
  }
});

function keydownHandler(event: KeyboardEvent) {
  // Ctrl + Enter to save
  if ((event.ctrlKey || event.metaKey) && event.key == "Enter") {
    saveHandler();
  }
  // Escape to exit edit mode
  if (event.key == "Escape") {
    closeHandler();
  }
}

// Helpers
function entityTooLargeToast(entityName) {
  toast.add(
    getToastOptions(
      t("note.tooLarge", { entity: entityName }),
      t("common.failed"),
      "error",
    ),
  );
}

function badFilenameToast(entityName) {
  toast.add(
    getToastOptions(
      t("note.invalidFilename"),
      t("note.invalidEntity", { entity: entityName }),
      "error",
    ),
  );
}

function draftStorageKey() {
  const userId = globalStore.currentUser?.id || "anonymous";
  const title = note.value.title || "new";
  return `draft:${userId}:${title}`;
}

function setBeforeUnloadConfirmation(enable = true) {
  if (enable) {
    window.onbeforeunload = () => {
      return true;
    };
  } else {
    window.onbeforeunload = null;
  }
}

function notifyNotesChanged() {
  window.dispatchEvent(new Event("mionote:notes-changed"));
}

function isContentChanged() {
  return (
    (toastEditor.value?.getHTML() || "") != baselineHtml ||
    persistedTitle() != note.value.title
  );
}

watch(() => props.title, init);
onMounted(init);
</script>
