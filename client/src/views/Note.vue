<template>
  <!-- Confirm Deletion Modal -->
  <ConfirmModal
    v-model="isDeleteModalVisible"
    :title="t('note.deleteTitle')"
    :message="t('note.deleteMessage', { title: note.title })"
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
    @confirm="saveHandler(true)"
    @reject="closeNote"
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
              entry.title || t("note.titlePlaceholder")
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
    class="mx-auto flex h-full max-w-none flex-col"
  >
    <section
      ref="workspaceElement"
      class="flex min-h-0 flex-1 flex-col bg-theme-canvas"
      :style="editorStyle"
    >
      <div
        class="relative h-12 shrink-0 items-center justify-between border-b border-theme-border sm:h-14 sm:pl-[var(--note-page-margin,1.5rem)] print:hidden"
        :class="!editMode || isMarkdownPreview ? 'flex' : 'hidden sm:flex'"
      >
        <div class="flex items-center gap-1">
          <Button
            variant="ghost"
            size="icon-sm"
            class="hidden sm:inline-flex"
            :title="t('nav.fullscreen')"
            :aria-label="t('nav.fullscreen')"
            @click="toggleFullscreen"
          >
            <Fullscreen class="h-5 w-5" />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            class="sm:h-10 sm:w-10"
            :title="t('nav.undo')"
            :aria-label="t('nav.undo')"
            :disabled="!editMode"
            @click="runEditorCommand('undo')"
          >
            <Undo2 class="h-4 w-4 sm:h-5 sm:w-5" />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            class="sm:h-10 sm:w-10"
            :title="t('nav.redo')"
            :aria-label="t('nav.redo')"
            :disabled="!editMode"
            @click="runEditorCommand('redo')"
          >
            <Redo2 class="h-4 w-4 sm:h-5 sm:w-5" />
          </Button>
          <span
            class="mx-0.5 h-5 w-px bg-theme-border sm:mx-1"
            aria-hidden="true"
          ></span>
          <label
            class="hidden items-center gap-1.5 text-xs font-medium text-theme-text-muted sm:flex"
            :class="{ 'text-theme-brand-strong': editorMode === 'markdown' }"
          >
            <span>{{ t("editor.markdown") }}</span>
            <Switch
              :model-value="editorMode === 'markdown'"
              :disabled="!editMode"
              @update:model-value="
                (value) => changeEditorMode(value ? 'markdown' : 'wysiwyg')
              "
            />
          </label>
        </div>

        <div class="flex shrink-0 items-center gap-0.5 pr-1 sm:gap-1">
          <Button
            variant="ghost"
            size="icon-sm"
            class="hidden sm:inline-flex sm:h-10 sm:w-10"
            :class="
              isFindPanelVisible
                ? 'bg-theme-background-elevated text-theme-text'
                : ''
            "
            :title="t('note.find')"
            :aria-label="t('note.find')"
            @click="togglePanel('find')"
          >
            <FileSearch class="h-4 w-4 sm:h-5 sm:w-5" />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            class="hidden sm:inline-flex sm:h-10 sm:w-10"
            :class="
              isStylePanelVisible
                ? 'bg-theme-background-elevated text-theme-text'
                : ''
            "
            :title="t('note.paperStyle')"
            :aria-label="t('note.paperStyle')"
            @click="togglePanel('style')"
          >
            <SlidersHorizontal class="h-4 w-4 sm:h-5 sm:w-5" />
          </Button>
          <span
            v-if="editMode && editorMode === 'markdown'"
            class="mx-1 h-5 w-px shrink-0 bg-theme-border"
            aria-hidden="true"
          ></span>
          <label
            v-if="editMode && editorMode === 'markdown'"
            class="hidden items-center gap-1.5 text-xs font-medium text-theme-text-muted sm:flex"
          >
            <span>{{ t("editor.preview") }}</span>
            <Switch
              :model-value="isMarkdownPreview"
              @update:model-value="setMarkdownPreview"
            />
          </label>
          <NoteMoreMenu :items="moreItems">
            <template #trigger>
              <Button
                variant="ghost"
                size="icon-sm"
                class="sm:h-10 sm:w-10"
                :aria-label="t('nav.menu')"
              >
                <MoreVertical class="h-4 w-4 sm:h-5 sm:w-5" />
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
        class="min-h-0 flex-1 overflow-y-auto px-4 pt-5 sm:pt-7"
        @contextmenu.capture="openNoteContextMenu"
      >
        <h1
          v-if="note.title"
          class="text-2xl font-semibold text-theme-text sm:text-3xl"
        >
          {{ note.title }}
        </h1>
        <ToastViewer :initialValue="note.content" class="toast-viewer pb-8" />
      </div>

      <div v-else class="note-editor-shell flex min-h-0 flex-1 flex-col">
        <EditorToolbar v-if="!isMarkdownPreview" :editor="toastEditor">
          <template #mobile-controls>
            <Button
              variant="ghost"
              size="icon-sm"
              :title="t('nav.undo')"
              :aria-label="t('nav.undo')"
              :disabled="!editMode"
              @click="runEditorCommand('undo')"
            >
              <Undo2 class="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="icon-sm"
              :title="t('nav.redo')"
              :aria-label="t('nav.redo')"
              :disabled="!editMode"
              @click="runEditorCommand('redo')"
            >
              <Redo2 class="h-4 w-4" />
            </Button>
            <label
              class="flex items-center gap-1.5 text-xs font-medium text-theme-text-muted"
              :class="{ 'text-theme-brand-strong': editorMode === 'markdown' }"
            >
              <span>{{ t("editor.markdown") }}</span>
              <Switch
                :model-value="editorMode === 'markdown'"
                :disabled="!editMode"
                @update:model-value="
                  (value) => changeEditorMode(value ? 'markdown' : 'wysiwyg')
                "
              />
            </label>
            <span
              v-if="editorMode === 'markdown'"
              class="mx-1 h-4 w-px shrink-0 bg-theme-border"
              aria-hidden="true"
            ></span>
            <label
              v-if="editorMode === 'markdown'"
              class="flex items-center gap-1.5 text-xs font-medium text-theme-text-muted"
            >
              <span>{{ t("editor.preview") }}</span>
              <Switch
                :model-value="isMarkdownPreview"
                @update:model-value="setMarkdownPreview"
              />
            </label>
            <div class="ml-auto shrink-0">
              <NoteMoreMenu :items="moreItems">
                <template #trigger>
                  <Button
                    variant="ghost"
                    size="icon-sm"
                    :aria-label="t('nav.menu')"
                  >
                    <MoreVertical class="h-4 w-4" />
                  </Button>
                </template>
              </NoteMoreMenu>
            </div>
          </template>
        </EditorToolbar>
        <div
          class="shrink-0 pl-[var(--note-page-margin,1.5rem)] pr-6 pt-4 sm:pr-10 sm:pt-6"
        >
          <input
            v-model="noteTitleField"
            type="text"
            class="w-full bg-transparent text-2xl font-semibold text-theme-text outline-none placeholder:text-theme-text-very-muted sm:text-3xl"
            :placeholder="t('note.title')"
            @keydown.enter.prevent
            @input="titleEdited"
          />
          <div class="mt-1.5 text-xs text-theme-text-muted">
            {{ t("note.characterCount", { count: editorCharacterCount }) }}
          </div>
        </div>
        <div
          v-if="isMarkdownPreview"
          class="min-h-0 flex-1 overflow-y-auto px-4 pb-8 sm:px-8"
          :style="pageMarginStyle"
        >
          <ToastViewer
            :initialValue="markdownPreviewContent"
            class="toast-viewer markdown-preview"
          />
        </div>
        <div
          v-show="!isMarkdownPreview"
          class="min-h-0 flex-1"
          @contextmenu.capture="openNoteContextMenu"
        >
          <ToastEditor
            ref="toastEditor"
            class="h-full"
            :initialValue="getInitialEditorValue()"
            :initialEditType="editorMode"
            :addImageBlobHook="addImageBlobHook"
            @change="editorChanged"
            @keydown="keydownHandler"
          />
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
/* Disable checkboxes in view mode. See https://github.com/nhn/tui.editor/issues/1087. */
.toast-viewer li.task-list-item {
  pointer-events: none;
}
.toast-viewer li.task-list-item a {
  pointer-events: auto;
}

.note-editor-shell .toastui-editor-md-container .toastui-editor,
.note-editor-shell .toastui-editor-md-container .CodeMirror,
.note-editor-shell .toastui-editor-ww-container .ProseMirror {
  padding: 0.5rem var(--note-page-margin, 1.5rem) 2rem;
}

.note-editor-shell .toastui-editor-main,
.note-editor-shell .toastui-editor-ww-container,
.note-editor-shell .toastui-editor-md-container {
  background-color: var(--note-paper-color, rgb(var(--theme-canvas)));
}

.note-editor-shell .toastui-editor-main {
  box-shadow: inset 0 0 24px rgb(61 56 41 / var(--note-paper-shadow-alpha, 0));
}

/* Keep Markdown source and rich-text editing on the same vertical rhythm. */
.note-editor-shell .toastui-editor-md-container .toastui-editor,
.note-editor-shell .toastui-editor-md-container .CodeMirror,
.note-editor-shell .toastui-editor-md-container .CodeMirror-lines,
.note-editor-shell .toastui-editor-ww-container .ProseMirror {
  font-size: 1rem;
  line-height: 1.6rem;
}

.note-editor-shell .toastui-editor-md-container .CodeMirror pre {
  margin: 0;
  line-height: 1.6rem;
}

.note-editor-shell .toastui-editor-ww-container .ProseMirror,
.note-editor-shell .toastui-editor-ww-container .ProseMirror p,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h1,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h2,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h3,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h4,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h5,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h6,
.note-editor-shell .toastui-editor-ww-container .ProseMirror li,
.note-editor-shell .toastui-editor-ww-container .ProseMirror blockquote,
.note-editor-shell .toastui-editor-ww-container .ProseMirror td,
.note-editor-shell .toastui-editor-ww-container .ProseMirror th,
.note-editor-shell .toastui-editor-ww-container .ProseMirror pre,
.note-editor-shell .toastui-editor-ww-container .ProseMirror code {
  line-height: 1.6rem;
}

.note-editor-shell .toastui-editor-ww-container .ProseMirror > p,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > h1,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > h2,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > h3,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > h4,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > h5,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > h6,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > ul,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > ol,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > blockquote,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > pre,
.note-editor-shell .toastui-editor-ww-container .ProseMirror > table {
  margin-top: 0;
  margin-bottom: 0;
}

.note-editor-shell .toastui-editor-ww-container .ProseMirror p,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h1,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h2,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h3,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h4,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h5,
.note-editor-shell .toastui-editor-ww-container .ProseMirror h6,
.note-editor-shell .toastui-editor-ww-container .ProseMirror li p {
  margin: 0;
}
</style>

<script setup lang="ts">
import {
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
import EditorToolbar from "../components/editor/EditorToolbar.vue";
import ToastEditor from "../components/editor/ToastEditor.vue";
import ToastViewer from "../components/editor/ToastViewer.vue";
import Button from "../components/ui/Button.vue";
import Dialog from "../components/ui/Dialog.vue";
import Input from "../components/ui/Input.vue";
import { useToast } from "../composables/useToast";
import { defaultNoteTitle } from "../utils/constants";
import { useGlobalStore } from "../stores/globalStore";
import { getToastOptions } from "../utils/helpers";
import { isCurrentTokenStored } from "../api/tokenStorage";
import NoteFindPanel from "../components/notes/NoteFindPanel.vue";
import NoteContextMenu from "../components/notes/NoteContextMenu.vue";
import NoteMoreMenu from "../components/notes/NoteMoreMenu.vue";
import NoteStylePanel from "../components/notes/NoteStylePanel.vue";
import SegmentedControl from "../components/ui/SegmentedControl.vue";
import Switch from "../components/ui/Switch.vue";
import {
  getNoteMetadata,
  removeNoteMetadata,
  setNoteMetadata,
} from "../utils/noteMetadata";
import type { EditorMode, NoteHistoryEntry, ToastEditorHandle } from "../types";

const props = defineProps<{ title?: string }>();

const { locale, t } = useI18n();
const canModify = computed(() => true);
let contentChangedTimeout: number | null = null;
let autoSaveInProgress = false;
let autoSaveQueued = false;
const editMode = ref(false);
const editorMode = ref<EditorMode>(loadDefaultEditorMode());
const isMarkdownPreview = ref(false);
const markdownPreviewContent = ref("");
const editorCharacterCount = ref(0);
const historyEntries = ref<NoteHistoryEntry[]>([]);
const globalStore = useGlobalStore();
const isFindPanelVisible = ref(false);
const findMatchCount = ref({ current: 0, total: 0 });
let activeFindKey = "";
let activeFindIndex = -1;
const isHistoryDialogVisible = ref(false);
const isSaveChangesModalVisible = ref(false);
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
  plain: "#ffffff",
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
const toastEditor = ref<ToastEditorHandle | null>(null);
const unsavedChanges = ref(false);
const workspaceElement = ref<HTMLElement | null>(null);
const editorStyle = computed(() => ({
  "--note-paper-color": paperColors[paperStyle.value] || paperColors.plain,
  "--note-paper-shadow-alpha": `${paperTexture.value / 900}`,
  "--note-page-margin": `${pageMargin.value}px`,
}));
const pageMarginStyle = computed(() => ({
  paddingLeft: `${pageMargin.value}px`,
  paddingRight: `${pageMargin.value}px`,
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
  if (!props.title && note.value.title === defaultNoteTitle && editMode.value) {
    router.replace({ name: "note", params: { title: defaultNoteTitle } });
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

  getNote(defaultNoteTitle)
    .then(open)
    .catch((error) => {
      if (error.response?.status !== 404) {
        fail(error);
        return;
      }
      createNote(defaultNoteTitle, "")
        .then(open)
        .catch((createError) => {
          if (createError.response?.status === 409) {
            getNote(defaultNoteTitle).then(open).catch(fail);
          } else {
            fail(createError);
          }
        });
    });
}

function runEditorCommand(command: string) {
  if (editMode.value) toastEditor.value?.exec(command);
}

function changeEditorMode(mode: string) {
  if (mode !== "markdown" && mode !== "wysiwyg") return;
  if (!editMode.value || !toastEditor.value) return;
  isMarkdownPreview.value = false;
  toastEditor.value.changeMode(mode);
  editorMode.value = mode;
}

function setMarkdownPreview(preview: boolean) {
  if (!editMode.value || editorMode.value !== "markdown") return;
  if (preview) {
    markdownPreviewContent.value = toastEditor.value?.getMarkdown() || "";
  }
  isMarkdownPreview.value = preview;
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
  newTitle.value = draft?.title || note.value.title || defaultNoteTitle;
  editorCharacterCount.value = countCharacters(
    draft?.content || note.value.content,
  );
  unsavedChanges.value = false;
  editorMode.value = loadDefaultEditorMode();
  isMarkdownPreview.value = false;
  editMode.value = true;
}

function getInitialEditorValue() {
  return loadDraft()?.content || note.value.content;
}

function editorChanged() {
  editorCharacterCount.value = countCharacters(
    toastEditor.value?.getMarkdown() || "",
  );
  startContentChangedTimeout();
}

// Inline title field: a brand-new note shows the placeholder instead of the
// default title; empty input restores the saved title instead of saving "".
const noteTitleField = computed({
  get: () =>
    isNewNote.value && newTitle.value === defaultNoteTitle
      ? ""
      : newTitle.value,
  set: (value: string) => {
    newTitle.value = value;
  },
});

function titleEdited() {
  const trimmed = newTitle.value.trim();
  if (!trimmed) {
    newTitle.value = isNewNote.value
      ? defaultNoteTitle
      : note.value.title || defaultNoteTitle;
    return;
  }
  newTitle.value = trimmed;
  startContentChangedTimeout();
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
  const isMouse =
    event.pointerType === "mouse" ||
    (!event.pointerType &&
      window.matchMedia("(hover: hover) and (pointer: fine)").matches);
  if (!note.value.title || !isMouse) return;

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
        ".toastui-editor-contents",
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
    newTitle.value === defaultNoteTitle ? "" : newTitle.value || "";
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
  const markdown = toastEditor.value?.getMarkdown() || note.value.content || "";
  const html = toastEditor.value?.getHTML() || markdownToExportHtml(markdown);
  const parser = document.createElement("div");
  parser.innerHTML = html;
  return {
    title: newTitle.value || note.value.title || "MioNote",
    markdown,
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

function markdownToExportHtml(markdown) {
  return markdown
    .split(/\n{2,}/)
    .map(
      (paragraph) =>
        `<p>${escapeExportHtml(paragraph).replace(/\n/g, "<br>")}</p>`,
    )
    .join("");
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
    newTitle.value = entry.title;
    toastEditor.value?.setMarkdown(entry.content);
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
  toastEditor.value?.highlightMatches(query, caseSensitive);
  if (focusEditor) {
    selectFindMatch(query, caseSensitive, match.matches[activeFindIndex]);
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
  if (isLocked.value || !toastEditor.value || !query) return;

  const content = toastEditor.value.getMarkdown();
  const source = caseSensitive ? content : content.toLocaleLowerCase();
  const needle = caseSensitive ? query : query.toLocaleLowerCase();
  const matches = findMatchOffsets(source, needle);
  if (!matches.length) {
    toast.add(getToastOptions(t("note.noMatches"), t("common.error"), "error"));
    return;
  }

  const findKey = `${caseSensitive}:${query}`;
  const replacementIndex = replaceEveryMatch
    ? -1
    : activeFindKey === findKey && activeFindIndex >= 0
      ? activeFindIndex
      : 0;
  let result = content;
  if (replaceEveryMatch) {
    for (let index = matches.length - 1; index >= 0; index -= 1) {
      const offset = matches[index];
      result =
        result.slice(0, offset) +
        replacement +
        result.slice(offset + query.length);
    }
  } else {
    const offset = matches[replacementIndex];
    result =
      result.slice(0, offset) +
      replacement +
      result.slice(offset + query.length);
  }
  toastEditor.value.setMarkdown(result);
  editorCharacterCount.value = countCharacters(result);
  activeFindKey = "";
  activeFindIndex = -1;
  findInNote({ query, caseSensitive, reset: true });
  startContentChangedTimeout();
}

function getFindMatch(query, caseSensitive, backwards, reset) {
  if (!query) return null;
  const content = toastEditor.value?.getMarkdown() || note.value.content || "";
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

function selectFindMatch(query, caseSensitive, markdownOffset) {
  const editor = toastEditor.value;
  if (!editor || !editMode.value) return;

  const visibleText = editor.getVisibleText() || "";
  const source = caseSensitive ? visibleText : visibleText.toLocaleLowerCase();
  const needle = caseSensitive ? query : query.toLocaleLowerCase();
  const occurrence = findMatchOffsets(
    caseSensitive
      ? editor.getMarkdown() || ""
      : (editor.getMarkdown() || "").toLocaleLowerCase(),
    needle,
  ).findIndex((offset) => offset === markdownOffset);
  const visibleOffsets = findMatchOffsets(source, needle);
  const visibleOffset =
    visibleOffsets[Math.max(0, occurrence)] ?? visibleOffsets[0];
  if (visibleOffset == null) return;

  // Select through the ProseMirror state so the selection survives editor
  // redraws; both mode containers share the same rendered-text mapping.
  editor.selectRange(visibleOffset, visibleOffset + query.length);
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
function saveHandler(close = false) {
  // Invalid Character Validation
  if (reservedFilenameCharacters.test(newTitle.value)) {
    badFilenameToast(t("note.title"));
    return;
  }

  // Save Note
  let newContent = toastEditor.value.getMarkdown();
  if (isNewNote.value) {
    saveNew(newTitle.value, newContent, close);
  } else {
    saveExisting(newTitle.value, newContent, close);
  }
}

function saveNew(newTitle, newContent, close = false) {
  createNote(newTitle, newContent)
    .then((data) => {
      clearDraft();
      note.value = data;
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
          noteSaveSuccess(close);
        });
    })
    .catch(noteSaveFailure);
}

function saveExisting(newTitle, newContent, close = false) {
  // Return if no changes
  if (newTitle == note.value.title && newContent == note.value.content) {
    noteSaveSuccess(close);
    return;
  }

  updateNote(note.value.title, newTitle, newContent)
    .then((data) => {
      clearDraft();
      note.value = data;
      recordHistory(data);
      notifyNotesChanged();
      router.replace({ name: "note", params: { title: note.value.title } });
      noteSaveSuccess(close);
    })
    .catch(noteSaveFailure);
}

function noteSaveFailure(error) {
  if (error.response?.status === 409) {
    toast.add(getToastOptions(t("note.duplicate"), t("common.error"), "error"));
  } else if (error.response?.status === 413) {
    entityTooLargeToast(t("note.title"));
  } else {
    apiErrorHandler(error, toast);
  }
}

function noteSaveSuccess(close = false) {
  unsavedChanges.value = false;
  if (close) {
    closeNote();
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

  const title = newTitle.value || defaultNoteTitle;
  const content = toastEditor.value.getMarkdown();
  if (title === note.value.title && content === note.value.content) return;

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
      recordHistory(data);
      notifyNotesChanged();
      if (isCreating || previousTitle !== data.title) {
        router.replace({ name: "note", params: { title: data.title } });
      }

      if (
        newTitle.value === data.title &&
        toastEditor.value?.getMarkdown() === data.content
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
        toast.add(
          getToastOptions(t("note.duplicate"), t("common.error"), "error"),
        );
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

// Note Closure
function closeHandler() {
  if (isContentChanged()) {
    isSaveChangesModalVisible.value = true;
  } else {
    closeNote();
  }
}

function closeNote() {
  clearDraft();
  editMode.value = false;
  if (isNewNote.value) {
    router.push({ name: "home" });
  } else {
    editMode.value = false;
  }
}

// Image Upload
function addImageBlobHook(
  file: File,
  callback: (url: string, altText?: string) => void,
) {
  const altTextInputValue = (
    document.getElementById("toastuiAltTextInput") as HTMLInputElement | null
  )?.value;

  // Upload the image then use the callback to insert the URL into the editor
  const request = postAttachment(file);
  if (!request) return;
  request.then(function (data) {
    if (data) {
      // If the user has entered an alt text, use it. Otherwise, use the filename returned by the API.
      const altText = altTextInputValue ? altTextInputValue : data.filename;
      callback(data.url, altText);
    }
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

function countCharacters(content: unknown) {
  return String(content ?? "").replace(/\s/g, "").length;
}

// Drafts
function saveDraft() {
  const content = toastEditor.value.getMarkdown();
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
    saveHandler(false);
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

function loadDefaultEditorMode(): EditorMode {
  return localStorage.getItem("defaultEditorMode") === "markdown"
    ? "markdown"
    : "wysiwyg";
}

function isContentChanged() {
  return (
    (newTitle.value || defaultNoteTitle) != note.value.title ||
    toastEditor.value.getMarkdown() != note.value.content
  );
}

watch(() => props.title, init);
onMounted(init);
</script>
