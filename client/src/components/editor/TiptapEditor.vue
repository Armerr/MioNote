<template>
  <div ref="editorRoot" class="mionote-tiptap-editor">
    <div class="mionote-tiptap-toolbar" role="toolbar" aria-label="编辑工具">
      <button
        type="button"
        title="标题"
        aria-label="标题"
        @mousedown.prevent
        @click="toggleHeading"
      >
        <Heading1 class="h-4 w-4" />
      </button>
      <button
        type="button"
        title="加粗"
        aria-label="加粗"
        @mousedown.prevent
        @click="run('bold')"
      >
        <Bold class="h-4 w-4" />
      </button>
      <button
        type="button"
        title="斜体"
        aria-label="斜体"
        @mousedown.prevent
        @click="run('italic')"
      >
        <Italic class="h-4 w-4" />
      </button>
      <button
        type="button"
        title="链接"
        aria-label="链接"
        @mousedown.prevent
        @click="addLink"
      >
        <LinkIcon class="h-4 w-4" />
      </button>
      <button
        type="button"
        title="无序列表"
        aria-label="无序列表"
        @mousedown.prevent
        @click="run('bulletList')"
      >
        <List class="h-4 w-4" />
      </button>
      <button
        type="button"
        title="待办事项"
        aria-label="待办事项"
        @mousedown.prevent
        @click="run('taskList')"
      >
        <ListTodo class="h-4 w-4" />
      </button>
      <button
        type="button"
        title="上传图片"
        aria-label="上传图片"
        @mousedown.prevent
        @click="imageInput?.click()"
      >
        <ImagePlus class="h-4 w-4" />
      </button>
      <button
        type="button"
        title="插入表格"
        aria-label="插入表格"
        @mousedown.prevent
        @click="insertTable"
      >
        <Table2 class="h-4 w-4" />
      </button>
      <span class="mionote-tiptap-toolbar-spacer"></span>
      <button
        type="button"
        title="撤销"
        aria-label="撤销"
        @mousedown.prevent
        @click="run('undo')"
      >
        <Undo2 class="h-4 w-4" />
      </button>
      <button
        type="button"
        title="重做"
        aria-label="重做"
        @mousedown.prevent
        @click="run('redo')"
      >
        <Redo2 class="h-4 w-4" />
      </button>
      <input
        ref="imageInput"
        type="file"
        accept="image/*"
        hidden
        @change="handleImageInput"
      />
    </div>

    <div
      ref="editorSurface"
      class="mionote-tiptap-surface"
      @scroll="updateTableControlsPosition"
    >
      <EditorContent
        v-show="mode === 'wysiwyg'"
        :editor="editorContent"
        class="mionote-tiptap-content"
      />
      <textarea
        v-show="mode === 'markdown'"
        ref="markdownInput"
        v-model="markdownSource"
        class="mionote-tiptap-markdown"
        spellcheck="false"
        aria-label="Markdown 源码"
        @beforeinput="captureMarkdownHistory"
        @input="handleMarkdownInput"
      ></textarea>
    </div>

    <div
      v-if="isTableActive && tableControlsStyle"
      class="mionote-tiptap-table-controls"
      :style="tableControlsStyle"
      role="toolbar"
      aria-label="表格操作"
      @mousedown.prevent
    >
      <button
        type="button"
        title="新增行"
        aria-label="新增行"
        @click="run('addRowAfter')"
      >
        +行
      </button>
      <button
        type="button"
        title="新增列"
        aria-label="新增列"
        @click="run('addColumnAfter')"
      >
        +列
      </button>
      <button
        type="button"
        title="删除行"
        aria-label="删除行"
        @click="run('deleteRow')"
      >
        -行
      </button>
      <button
        type="button"
        title="删除列"
        aria-label="删除列"
        @click="run('deleteColumn')"
      >
        -列
      </button>
      <button
        type="button"
        title="删除表格"
        aria-label="删除表格"
        @click="run('deleteTable')"
      >
        删表
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Extension } from "@tiptap/core";
import { Plugin, PluginKey } from "@tiptap/pm/state";
import { Decoration, DecorationSet } from "@tiptap/pm/view";
import { Editor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import { Markdown } from "@tiptap/markdown";
import { TableKit } from "@tiptap/extension-table";
import Image from "@tiptap/extension-image";
import Link from "@tiptap/extension-link";
import Placeholder from "@tiptap/extension-placeholder";
import TaskList from "@tiptap/extension-task-list";
import TaskItem from "@tiptap/extension-task-item";
import UniqueID from "@tiptap/extension-unique-id";
import Highlight from "@tiptap/extension-highlight";
import TextAlign from "@tiptap/extension-text-align";
import {
  Bold,
  Heading1,
  ImagePlus,
  Italic,
  Link as LinkIcon,
  List,
  ListTodo,
  Redo2,
  Table2,
  Undo2,
} from "lucide-vue-next";
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from "vue";

import type {
  EditorMode,
  EditorSelection,
  EditorSelectionStyle,
} from "../../types";

const props = defineProps({
  initialValue: { type: String, default: "" },
  initialEditType: { type: String, default: "wysiwyg" },
  addImageBlobHook: Function,
});

const emit = defineEmits(["change", "keydown"]);
const editorRoot = ref<HTMLElement | null>(null);
const editorSurface = ref<HTMLElement | null>(null);
const imageInput = ref<HTMLInputElement | null>(null);
const markdownInput = ref<HTMLTextAreaElement | null>(null);
const editor = ref<Editor | null>(null);
const editorContent = computed(() => editor.value as unknown as Editor);
const mode = ref<EditorMode>(
  props.initialEditType === "markdown" ? "markdown" : "wysiwyg",
);
const markdownSource = ref(props.initialValue || "");
let dragSource: { from: number; size: number } | null = null;
let editorDom: HTMLElement | null = null;

type MarkdownHistoryEntry = {
  value: string;
  selectionStart: number;
  selectionEnd: number;
};

const markdownUndoStack = ref<MarkdownHistoryEntry[]>([]);
const markdownRedoStack = ref<MarkdownHistoryEntry[]>([]);
const maximumMarkdownHistoryEntries = 200;

const topLevelTypes = [
  "paragraph",
  "heading",
  "blockquote",
  "bulletList",
  "orderedList",
  "taskList",
  "codeBlock",
  "horizontalRule",
  "image",
  "table",
];

const findHighlightKey = new PluginKey<DecorationSet>("mionoteFindHighlight");
const FindHighlight = Extension.create({
  name: "findHighlight",
  addProseMirrorPlugins() {
    return [
      new Plugin<DecorationSet>({
        key: findHighlightKey,
        state: {
          init: () => DecorationSet.empty,
          apply(transaction, decorations) {
            const nextDecorations = transaction.getMeta(findHighlightKey);
            return (
              nextDecorations ??
              decorations.map(transaction.mapping, transaction.doc)
            );
          },
        },
        props: {
          decorations(state) {
            return findHighlightKey.getState(state);
          },
        },
      }),
    ];
  },
});

const extensions = [
  StarterKit.configure({ link: false }),
  Link.configure({ openOnClick: false, autolink: true }),
  Image.configure({ inline: false, allowBase64: true }),
  TableKit.configure({ table: { resizable: true } }),
  TaskList,
  TaskItem.configure({ nested: true }),
  Highlight.configure({ multicolor: true }),
  TextAlign.configure({ types: ["heading", "paragraph"] }),
  Placeholder.configure({ placeholder: "开始记录..." }),
  UniqueID.configure({ types: topLevelTypes }),
  Markdown,
  FindHighlight,
];

const tableActive = ref(false);
const tableControlsStyle = ref<Record<string, string> | null>(null);
const isTableActive = computed(
  () => mode.value === "wysiwyg" && tableActive.value,
);

function syncTableState() {
  const current = editor.value;
  if (!current) {
    tableActive.value = false;
    tableControlsStyle.value = null;
    return;
  }
  const { $from } = current.state.selection;
  tableActive.value = Boolean(
    $from &&
    Array.from({ length: $from.depth + 1 }, (_, depth) =>
      $from.node(depth),
    ).some((node) => node?.type.name === "table"),
  );
  if (tableActive.value) nextTick(updateTableControlsPosition);
  else tableControlsStyle.value = null;
}

function getSelectedTableElement() {
  const current = editor.value;
  if (!current) return null;
  const { node } = current.view.domAtPos(current.state.selection.from);
  const element =
    node.nodeType === Node.ELEMENT_NODE
      ? (node as Element)
      : node.parentElement;
  return element?.closest(".tableWrapper, table") as HTMLElement | null;
}

function updateTableControlsPosition() {
  if (!isTableActive.value) return;
  const root = editorRoot.value;
  const surface = editorSurface.value;
  const table = getSelectedTableElement();
  if (!root || !surface || !table) {
    tableControlsStyle.value = null;
    return;
  }

  const rootRect = root.getBoundingClientRect();
  const surfaceRect = surface.getBoundingClientRect();
  const tableRect = table.getBoundingClientRect();
  const right = Math.min(tableRect.right, rootRect.right - 8);
  const top = Math.max(tableRect.top + 8, surfaceRect.top + 8);

  tableControlsStyle.value = {
    left: `${Math.max(8, right - rootRect.left - 8)}px`,
    top: `${top - rootRect.top}px`,
  };
}

function getMarkdown() {
  if (mode.value === "markdown") return markdownSource.value;
  return editor.value?.getMarkdown() || "";
}

function getHTML() {
  return editor.value?.getHTML() || "";
}

function getMarkdownHistoryEntry(): MarkdownHistoryEntry {
  const input = markdownInput.value;
  const fallbackPosition = markdownSource.value.length;
  return {
    value: markdownSource.value,
    selectionStart: input?.selectionStart ?? fallbackPosition,
    selectionEnd: input?.selectionEnd ?? fallbackPosition,
  };
}

function appendMarkdownHistory(
  stack: MarkdownHistoryEntry[],
  entry: MarkdownHistoryEntry,
) {
  stack.push(entry);
  if (stack.length > maximumMarkdownHistoryEntries) stack.shift();
}

function captureMarkdownHistory() {
  const entry = getMarkdownHistoryEntry();
  const lastEntry = markdownUndoStack.value[markdownUndoStack.value.length - 1];
  if (
    lastEntry?.value === entry.value &&
    lastEntry.selectionStart === entry.selectionStart &&
    lastEntry.selectionEnd === entry.selectionEnd
  ) {
    return;
  }
  appendMarkdownHistory(markdownUndoStack.value, entry);
  markdownRedoStack.value = [];
}

function resetMarkdownHistory() {
  markdownUndoStack.value = [];
  markdownRedoStack.value = [];
}

function syncMarkdownSource(emitChange = true) {
  editor.value?.commands.setContent(markdownSource.value, {
    contentType: "markdown",
    emitUpdate: false,
  });
  if (emitChange) emit("change");
}

function restoreMarkdownHistory(entry: MarkdownHistoryEntry) {
  markdownSource.value = entry.value;
  syncMarkdownSource(false);
  nextTick(() => {
    const input = markdownInput.value;
    if (!input) return;
    input.focus();
    input.setSelectionRange(entry.selectionStart, entry.selectionEnd);
  });
  emit("change");
}

function undoMarkdown() {
  const entry = markdownUndoStack.value.pop();
  if (!entry) return;
  appendMarkdownHistory(markdownRedoStack.value, getMarkdownHistoryEntry());
  restoreMarkdownHistory(entry);
}

function redoMarkdown() {
  const entry = markdownRedoStack.value.pop();
  if (!entry) return;
  appendMarkdownHistory(markdownUndoStack.value, getMarkdownHistoryEntry());
  restoreMarkdownHistory(entry);
}

function setMarkdown(markdown: string) {
  markdownSource.value = markdown || "";
  resetMarkdownHistory();
  syncMarkdownSource();
}

function focus() {
  if (mode.value === "markdown") markdownInput.value?.focus();
  else editor.value?.commands.focus();
}

function isWysiwygMode() {
  return mode.value === "wysiwyg";
}

function run(command: string) {
  if (mode.value === "markdown") {
    if (command === "undo") undoMarkdown();
    else if (command === "redo") redoMarkdown();
    return;
  }
  const current = editor.value;
  if (!current) return;
  const chain = current.chain().focus();
  const commandMap: Record<string, () => boolean> = {
    bold: () => chain.toggleBold().run(),
    italic: () => chain.toggleItalic().run(),
    bulletList: () => chain.toggleBulletList().run(),
    taskList: () => chain.toggleTaskList().run(),
    undo: () => chain.undo().run(),
    redo: () => chain.redo().run(),
    addRowAfter: () => chain.addRowAfter().run(),
    addColumnAfter: () => chain.addColumnAfter().run(),
    deleteRow: () => chain.deleteRow().run(),
    deleteColumn: () => chain.deleteColumn().run(),
    deleteTable: () => chain.deleteTable().run(),
  };
  if (commandMap[command]) commandMap[command]();
  syncTableState();
  emit("change");
}

function toggleHeading() {
  if (!editor.value || mode.value !== "wysiwyg") return;
  editor.value.chain().focus().toggleHeading({ level: 2 }).run();
  emit("change");
}

function addLink() {
  if (!editor.value || mode.value !== "wysiwyg") return;
  const previous = editor.value.getAttributes("link").href || "https://";
  const href = window.prompt("链接地址", previous);
  if (href === null) return;
  if (!href.trim()) editor.value.chain().focus().unsetLink().run();
  else editor.value.chain().focus().setLink({ href: href.trim() }).run();
  emit("change");
}

function insertTable() {
  if (!editor.value || mode.value !== "wysiwyg") return;
  editor.value
    .chain()
    .focus()
    .insertTable({ rows: 2, cols: 2, withHeaderRow: true })
    .run();
  syncTableState();
  emit("change");
}

function insertText(text: string) {
  if (!text) return;
  if (mode.value === "markdown") {
    const input = markdownInput.value;
    if (!input) return;
    const start = input.selectionStart;
    const end = input.selectionEnd;
    captureMarkdownHistory();
    markdownSource.value = `${markdownSource.value.slice(0, start)}${text}${markdownSource.value.slice(end)}`;
    nextTick(() => {
      input.setSelectionRange(start + text.length, start + text.length);
    });
    handleMarkdownInput();
  } else {
    editor.value?.chain().focus().insertContent(text).run();
    emit("change");
  }
}

function replaceSelection(text: string) {
  if (mode.value === "markdown") {
    insertText(text);
    return;
  }
  editor.value?.chain().focus().insertContent(text).run();
  emit("change");
}

function deleteSelection() {
  if (mode.value === "markdown") {
    const input = markdownInput.value;
    if (!input || input.selectionStart === input.selectionEnd) return;
    const start = input.selectionStart;
    const end = input.selectionEnd;
    captureMarkdownHistory();
    markdownSource.value = `${markdownSource.value.slice(0, start)}${markdownSource.value.slice(end)}`;
    nextTick(() => input.setSelectionRange(start, start));
    handleMarkdownInput();
  } else {
    editor.value?.chain().focus().deleteSelection().run();
    emit("change");
  }
}

function getSelectedText() {
  if (mode.value === "markdown") {
    const input = markdownInput.value;
    return input
      ? markdownSource.value.slice(input.selectionStart, input.selectionEnd)
      : "";
  }
  const current = editor.value;
  if (!current) return "";
  const { from, to } = current.state.selection;
  return current.state.doc.textBetween(from, to, "\n", "\ufffc");
}

function visibleOffsetToPos(target: number) {
  const current = editor.value;
  if (!current) return 1;
  let result = 1;
  let consumed = 0;
  current.state.doc.descendants((node, pos) => {
    if (node.isText) {
      const end = consumed + node.text!.length;
      if (target <= end) {
        result = pos + Math.max(0, target - consumed);
        return false;
      }
      consumed = end;
    } else if (node.isTextblock) {
      if (consumed > 0) consumed += 1;
    }
    return true;
  });
  return result;
}

function getSelection(): EditorSelection | undefined {
  if (mode.value === "markdown") {
    const input = markdownInput.value;
    return input ? [input.selectionStart, input.selectionEnd] : undefined;
  }
  const current = editor.value;
  if (!current) return undefined;
  const { from, to } = current.state.selection;
  return [
    current.state.doc.textBetween(0, from, "\n", "\ufffc").length,
    current.state.doc.textBetween(0, to, "\n", "\ufffc").length,
  ];
}

function selectRange(start: number, end: number) {
  if (mode.value === "markdown") {
    const input = markdownInput.value;
    input?.focus();
    input?.setSelectionRange(start, end);
    return;
  }
  const current = editor.value;
  if (!current) return;
  const from = visibleOffsetToPos(start);
  const to = visibleOffsetToPos(end);
  current.commands.setTextSelection({ from, to: Math.max(from, to) });
  current.commands.scrollIntoView();
  current.commands.focus();
}

function setSelection(start: number, end: number) {
  selectRange(start, end);
}

function getVisibleText() {
  if (mode.value === "markdown") return markdownSource.value;
  const current = editor.value;
  return (
    current?.state.doc.textBetween(
      0,
      current.state.doc.content.size,
      "\n",
      "\ufffc",
    ) || ""
  );
}

function highlightMatches(query: string, caseSensitive = false) {
  const current = editor.value;
  if (!current) return;
  if (!query) {
    clearMatches();
    return;
  }

  const doc = current.state.doc;
  const text = doc.textBetween(0, doc.content.size, "\n", "\ufffc");
  const haystack = caseSensitive ? text : text.toLocaleLowerCase();
  const needle = caseSensitive ? query : query.toLocaleLowerCase();
  const decorations: Decoration[] = [];
  let offset = haystack.indexOf(needle);
  let guard = 0;

  while (offset >= 0 && guard < 1000) {
    guard += 1;
    const from = visibleOffsetToPos(offset);
    const to = visibleOffsetToPos(offset + needle.length);
    if (to > from) {
      decorations.push(
        Decoration.inline(from, to, { class: "mionote-find-match" }),
      );
    }
    offset = haystack.indexOf(needle, offset + needle.length);
  }

  current.view.dispatch(
    current.state.tr.setMeta(
      findHighlightKey,
      DecorationSet.create(
        doc as unknown as Parameters<typeof DecorationSet.create>[0],
        decorations,
      ),
    ),
  );
}

function clearMatches() {
  const current = editor.value;
  if (!current) return;
  current.view.dispatch(
    current.state.tr.setMeta(findHighlightKey, DecorationSet.empty),
  );
}

function getSelectionStyle(): EditorSelectionStyle | null {
  if (!editor.value || mode.value !== "wysiwyg") return null;
  return {
    bold: editor.value.isActive("bold"),
    italic: editor.value.isActive("italic"),
    strike: editor.value.isActive("strike"),
    underline: editor.value.isActive("underline"),
    inlineStyles: {
      color: editor.value.getAttributes("textStyle").color || "",
      "font-size": "",
    },
  };
}

function applyInlineStyle(tag: string, style: Record<string, string> = {}) {
  if (!editor.value || mode.value !== "wysiwyg") return false;
  const chain = editor.value.chain().focus();
  if (tag === "strong") chain.toggleBold().run();
  else if (tag === "em") chain.toggleItalic().run();
  else if (tag === "strike") chain.toggleStrike().run();
  else if (tag === "u") chain.toggleUnderline().run();
  else if (tag === "span" && style.color) {
    editor.value.commands.insertContent(
      `<span style="color:${style.color}">${getSelectedText()}</span>`,
    );
  } else if (tag === "mark") {
    editor.value.commands.toggleHighlight?.({
      color: style["background-color"],
    });
  }
  emit("change");
  return true;
}

function applyAlignment(alignment: string) {
  if (!editor.value || mode.value !== "wysiwyg") return false;
  editor.value.commands.setTextAlign?.(alignment);
  emit("change");
  return true;
}

function applyCopiedStyle(style: EditorSelectionStyle) {
  if (style.bold) editor.value?.chain().focus().toggleBold().run();
  if (style.italic) editor.value?.chain().focus().toggleItalic().run();
  if (style.underline) editor.value?.chain().focus().toggleUnderline().run();
  if (style.strike) editor.value?.chain().focus().toggleStrike().run();
  emit("change");
  return Boolean(getSelectedText());
}

function exec(command: string, payload?: unknown) {
  if (command === "selectAll") {
    if (mode.value === "markdown") markdownInput.value?.select();
    else editor.value?.commands.selectAll();
  } else if (command === "addImage" && payload && typeof payload === "object") {
    const image = payload as { imageUrl?: string; altText?: string };
    editor.value
      ?.chain()
      .focus()
      .setImage({ src: image.imageUrl || "", alt: image.altText || "image" })
      .run();
    emit("change");
  } else run(command);
}

function addImageFile(file: File) {
  if (!file) return;
  const callback = (url: string, altText = file.name) => {
    if (mode.value === "markdown") {
      insertText(`![${altText}](${url})`);
    } else {
      editor.value?.chain().focus().setImage({ src: url, alt: altText }).run();
      emit("change");
    }
  };
  if (props.addImageBlobHook) props.addImageBlobHook(file, callback);
  else {
    const reader = new FileReader();
    reader.onload = () => callback(String(reader.result || ""));
    reader.readAsDataURL(file);
  }
}

function addMediaFile(file: File) {
  if (file?.type.startsWith("image/")) addImageFile(file);
}

function addAttachmentFile(file: File) {
  if (!file) return;
  const callback = (url: string, filename = file.name) =>
    insertText(`[${filename}](${url})`);
  if (props.addImageBlobHook) props.addImageBlobHook(file, callback);
}

function handleImageInput(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (file) addImageFile(file);
  input.value = "";
}

function handleMarkdownInput() {
  syncMarkdownSource();
}

function changeMode(nextMode: EditorMode) {
  if (!editor.value || mode.value === nextMode) return;
  if (nextMode === "markdown")
    markdownSource.value = editor.value.getMarkdown();
  else
    editor.value.commands.setContent(markdownSource.value, {
      contentType: "markdown",
      emitUpdate: false,
    });
  mode.value = nextMode;
  nextTick(() => {
    syncTableState();
    focus();
  });
}

function decorateBlocks() {
  const root = editor.value?.view.dom;
  if (!root) return;
  Array.from(root.children).forEach((child) => {
    const element = child as HTMLElement;
    element.setAttribute("data-block", "true");
    element.setAttribute("draggable", "true");
  });
}

function onDragStart(event: DragEvent) {
  const target = event.target as HTMLElement | null;
  const root = editor.value?.view.dom;
  const current = editor.value;
  const block = target?.closest("[data-block]") as HTMLElement | null;
  if (!block || !root || !current || block.parentElement !== root) return;
  const index = Array.from(root.children).indexOf(block);
  const from = current.state.doc.content.content
    .slice(0, index)
    .reduce((sum, node) => sum + node.nodeSize, 0);
  const node = current.state.doc.child(index);
  dragSource = { from, size: node.nodeSize };
  event.dataTransfer?.setData("text/plain", node.attrs.id || "block");
  if (event.dataTransfer) event.dataTransfer.effectAllowed = "move";
}

function onDragOver(event: DragEvent) {
  if (dragSource) event.preventDefault();
}

function onDrop(event: DragEvent) {
  const root = editor.value?.view.dom;
  const current = editor.value;
  if (!dragSource || !root || !current) return;
  const target = (event.target as HTMLElement | null)?.closest("[data-block]");
  if (!target || target.parentElement !== root) return;
  event.preventDefault();
  const targetIndex = Array.from(root.children).indexOf(target);
  const sourceIndex = current.state.doc.content.content.findIndex(
    (_, index) => {
      const from = current.state.doc.content.content
        .slice(0, index)
        .reduce((sum, node) => sum + node.nodeSize, 0);
      return from === dragSource!.from;
    },
  );
  if (sourceIndex < 0 || sourceIndex === targetIndex) {
    dragSource = null;
    return;
  }
  const node = current.state.doc.child(sourceIndex);
  let insertAt = current.state.doc.content.content
    .slice(0, targetIndex)
    .reduce((sum, item) => sum + item.nodeSize, 0);
  if (targetIndex > sourceIndex) insertAt -= node.nodeSize;
  current.view.dispatch(
    current.state.tr
      .delete(dragSource.from, dragSource.from + node.nodeSize)
      .insert(insertAt, node),
  );
  dragSource = null;
  emit("change");
}

onMounted(() => {
  const instance = new Editor({
    extensions,
    content: props.initialValue || "",
    contentType: "markdown",
    autofocus: false,
    onUpdate: () => {
      if (mode.value === "wysiwyg") emit("change");
      syncTableState();
      decorateBlocks();
    },
    onSelectionUpdate: () => {
      syncTableState();
      decorateBlocks();
    },
    onCreate: () =>
      nextTick(() => {
        syncTableState();
        decorateBlocks();
      }),
  });
  editor.value = instance;
  editorDom = instance.view.dom;
  editorDom.addEventListener("keydown", (event) => emit("keydown", event));
  editorDom.addEventListener("dragstart", onDragStart);
  editorDom.addEventListener("dragover", onDragOver);
  editorDom.addEventListener("drop", onDrop);
  window.addEventListener("resize", updateTableControlsPosition);
  decorateBlocks();
});

onBeforeUnmount(() => {
  editorDom?.removeEventListener("dragstart", onDragStart);
  editorDom?.removeEventListener("dragover", onDragOver);
  editorDom?.removeEventListener("drop", onDrop);
  window.removeEventListener("resize", updateTableControlsPosition);
  editor.value?.destroy();
  editor.value = null;
});

defineExpose({
  addAttachmentFile,
  addImageFile,
  addMediaFile,
  applyAlignment,
  applyCopiedStyle,
  applyInlineStyle,
  changeMode,
  clearMatches,
  deleteSelection,
  exec,
  focus,
  getHTML,
  getMarkdown,
  getSelectedText,
  getSelection,
  getSelectionStyle,
  getVisibleText,
  highlightMatches,
  insertText,
  isWysiwygMode,
  replaceSelection,
  selectRange,
  setMarkdown,
  setSelection,
});
</script>

<style>
.mionote-tiptap-editor {
  position: relative;
  display: flex;
  height: 100%;
  min-height: 0;
  flex-direction: column;
  color: rgb(var(--theme-text));
  background: var(--note-paper-color, rgb(var(--theme-canvas)));
}

.mionote-tiptap-toolbar {
  display: flex;
  min-height: 2.75rem;
  flex: 0 0 auto;
  align-items: center;
  gap: 0.125rem;
  overflow: hidden;
  white-space: nowrap;
  border-bottom: 1px solid rgb(var(--theme-border));
  padding: 0.25rem var(--note-page-margin, 1.5rem);
  background: rgb(var(--theme-canvas));
}

.mionote-tiptap-toolbar button {
  display: inline-flex;
  height: 2rem;
  min-width: 2rem;
  flex: 0 0 auto;
  align-items: center;
  justify-content: center;
  gap: 0.125rem;
  border: 0;
  border-radius: 0.375rem;
  padding: 0 0.375rem;
  color: rgb(var(--theme-text-muted));
  background: transparent;
  cursor: pointer;
}

.mionote-tiptap-toolbar button:hover,
.mionote-tiptap-toolbar button:focus-visible {
  color: rgb(var(--theme-text));
  background: rgb(var(--theme-background-elevated));
  outline: none;
}

.mionote-tiptap-toolbar-divider {
  width: 1px;
  height: 1.25rem;
  flex: 0 0 auto;
  margin: 0 0.25rem;
  background: rgb(var(--theme-border));
}

.mionote-tiptap-toolbar-spacer {
  min-width: 0.25rem;
  flex: 1 1 auto;
}

.mionote-tiptap-surface {
  min-height: 0;
  flex: 1 1 auto;
  overflow: auto;
}

.mionote-tiptap-table-controls {
  position: absolute;
  z-index: 4;
  display: flex;
  align-items: center;
  gap: 0.125rem;
  transform: translateX(-100%);
  border: 1px solid rgb(var(--theme-border));
  border-radius: 0.375rem;
  padding: 0.1875rem;
  color: rgb(var(--theme-text));
  background: var(--note-paper-color, rgb(var(--theme-canvas)));
  box-shadow: 0 6px 18px rgb(0 0 0 / 0.12);
}

.mionote-tiptap-table-controls button {
  display: inline-flex;
  height: 1.75rem;
  min-width: 1.75rem;
  align-items: center;
  justify-content: center;
  border: 0;
  border-radius: 0.25rem;
  padding: 0 0.3125rem;
  color: rgb(var(--theme-text-muted));
  background: transparent;
  font-size: 0.75rem;
  line-height: 1;
  cursor: pointer;
}

.mionote-tiptap-table-controls button:hover,
.mionote-tiptap-table-controls button:focus-visible {
  color: rgb(var(--theme-text));
  background: rgb(var(--theme-background-elevated));
  outline: none;
}

.mionote-tiptap-table-controls button[title="删除表格"] {
  color: rgb(var(--theme-danger, 190 24 93));
}

.mionote-tiptap-content,
.mionote-tiptap-markdown {
  display: block;
  width: 100%;
  min-height: 100%;
  box-sizing: border-box;
  border: 0;
  padding: 0.75rem var(--note-page-margin, 1.5rem) 2rem;
  color: rgb(var(--theme-text));
  background: transparent;
}

.mionote-tiptap-markdown {
  resize: none;
  outline: none;
  font: inherit;
  line-height: 1.65;
  white-space: pre-wrap;
}

.mionote-tiptap-content .tiptap {
  min-height: 100%;
  outline: none;
  line-height: 1.65;
}

.mionote-tiptap-content .tiptap > [data-block] {
  position: relative;
  border-radius: 0.25rem;
}

.mionote-tiptap-content .tiptap > [data-block]:hover {
  background: rgb(var(--theme-background-elevated) / 0.24);
}

.mionote-tiptap-content .tiptap > [data-block]::before {
  position: absolute;
  top: 0.55rem;
  left: -1.125rem;
  width: 0.75rem;
  height: 0.9rem;
  content: "\2807";
  color: rgb(var(--theme-text-very-muted));
  opacity: 0;
  cursor: grab;
  transition: opacity 120ms ease;
}

.mionote-tiptap-content .tiptap > [data-block]:hover::before {
  opacity: 1;
}

.mionote-tiptap-content .tiptap table {
  width: 100%;
  border-collapse: collapse;
  table-layout: fixed;
}

.mionote-tiptap-content .tiptap th,
.mionote-tiptap-content .tiptap td {
  min-width: 5rem;
  border: 1px solid rgb(var(--theme-border));
  padding: 0.45rem 0.6rem;
  vertical-align: top;
}

.mionote-tiptap-content .tiptap th {
  background: rgb(var(--theme-background-elevated));
}

.mionote-tiptap-content .tiptap img {
  max-width: 100%;
  height: auto;
}

.mionote-tiptap-content .tiptap .selectedCell {
  background: rgb(var(--theme-brand) / 0.12);
}

@media (max-width: 639px) {
  .mionote-tiptap-toolbar {
    min-height: 2.5rem;
    padding: 0.125rem 0.5rem;
  }

  .mionote-tiptap-toolbar button {
    min-width: 1.75rem;
    height: 2rem;
    padding: 0 0.25rem;
  }

  .mionote-tiptap-toolbar-divider,
  .mionote-tiptap-toolbar-spacer,
  .mionote-tiptap-toolbar button[title="链接"],
  .mionote-tiptap-toolbar button[title="待办事项"],
  .mionote-tiptap-toolbar button[title="撤销"],
  .mionote-tiptap-toolbar button[title="重做"] {
    display: none;
  }

  .mionote-tiptap-table-controls {
    gap: 0;
    padding: 0.125rem;
  }

  .mionote-tiptap-table-controls button {
    height: 1.625rem;
    min-width: 1.625rem;
    padding: 0 0.25rem;
    font-size: 0.6875rem;
  }

  .mionote-tiptap-content,
  .mionote-tiptap-markdown {
    padding-right: 0.75rem;
    padding-left: 0.75rem;
  }
}
</style>
