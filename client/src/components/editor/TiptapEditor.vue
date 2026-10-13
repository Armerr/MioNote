<template>
  <div ref="editorRoot" class="mionote-tiptap-editor">
    <div ref="editorSurface" class="mionote-tiptap-surface">
      <EditorContent :editor="editorContent" class="mionote-rich-text editor" />
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
import { Decoration, DecorationSet } from "@tiptap/pm/view";
import { Editor, EditorContent } from "@tiptap/vue-3";
import { addColumn, addRow, selectedRect } from "@tiptap/pm/tables";
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from "vue";

import { createNoteExtensions, findHighlightKey } from "./extensions";
import type { EditorSelection, EditorSelectionStyle } from "../../types";
import "./richTextContent.css";

const props = defineProps({
  initialValue: { type: String, default: "" },
  addImageBlobHook: Function,
});

const emit = defineEmits(["change", "keydown", "ready"]);
const editorRoot = ref<HTMLElement | null>(null);
const editorSurface = ref<HTMLElement | null>(null);
const editor = ref<Editor | null>(null);
const editorContent = computed(() => editor.value as unknown as Editor);
let dragSource: { from: number; size: number } | null = null;
let editorDom: HTMLElement | null = null;

const tableActive = ref(false);
const tableControlsStyle = ref<Record<string, string> | null>(null);
const isTableActive = computed(() => tableActive.value);

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
  const table = getSelectedTableElement();
  if (!root || !table) {
    tableControlsStyle.value = null;
    return;
  }

  const rootRect = root.getBoundingClientRect();
  const tableRect = table.getBoundingClientRect();
  const barHeight = 36;
  // Sit just above the table's top-right corner (outside the table) so the bar
  // never covers cell content. If there is no room above, drop it just below.
  let top = tableRect.top - rootRect.top - barHeight - 6;
  if (top < 6) top = tableRect.bottom - rootRect.top + 6;
  const rightEdge = Math.min(tableRect.right, rootRect.right - 8);

  tableControlsStyle.value = {
    top: `${top}px`,
    right: `${Math.max(8, rootRect.right - rightEdge)}px`,
  };
}

function getHTML() {
  return editor.value?.getHTML() || "";
}

function setHTML(html: string) {
  editor.value?.commands.setContent(html || "", { emitUpdate: false });
  emit("change");
}

function focus() {
  editor.value?.commands.focus();
}

// "Add column/row" appends at the far right/bottom of the table rather than
// next to the current cell, matching the toolbar's "新增列 / 新增行" wording.
// The prosemirror-tables helpers carry their own prosemirror-state types, which
// don't structurally match the copies Tiptap re-exports, so the state/tr are
// cast at the boundary.
function appendColumnToTable() {
  return appendTableTrack("column");
}

function appendRowToTable() {
  return appendTableTrack("row");
}

function appendTableTrack(axis: "column" | "row") {
  const current = editor.value;
  if (!current) return false;
  const rect = selectedRect(
    current.state as unknown as Parameters<typeof selectedRect>[0],
  );
  if (!rect) return false;
  const tr = current.state.tr as unknown as Parameters<typeof addColumn>[0];
  const next =
    axis === "column"
      ? addColumn(tr, rect, rect.map.width)
      : addRow(tr, rect, rect.map.height);
  current.view.dispatch(
    next as unknown as Parameters<typeof current.view.dispatch>[0],
  );
  return true;
}

function run(command: string, payload?: any) {
  const current = editor.value;
  if (!current) return;
  const chain = current.chain().focus();
  const commandMap: Record<string, () => boolean> = {
    paragraph: () => chain.setParagraph().run(),
    bold: () => chain.toggleBold().run(),
    italic: () => chain.toggleItalic().run(),
    strike: () => chain.toggleStrike().run(),
    underline: () => chain.toggleUnderline().run(),
    bulletList: () => chain.toggleBulletList().run(),
    orderedList: () => chain.toggleOrderedList().run(),
    taskList: () => chain.toggleTaskList().run(),
    undo: () => chain.undo().run(),
    redo: () => chain.redo().run(),
    addRowAfter: () => appendRowToTable(),
    addColumnAfter: () => appendColumnToTable(),
    deleteRow: () => chain.deleteRow().run(),
    deleteColumn: () => chain.deleteColumn().run(),
    deleteTable: () => chain.deleteTable().run(),
  };

  let handled = false;
  if (command === "heading") {
    handled = current
      .chain()
      .focus()
      .toggleHeading({ level: payload?.level ?? 1 })
      .run();
  } else if (command === "addTable") {
    handled = current
      .chain()
      .focus()
      .insertTable({
        rows: payload?.rowCount ?? 2,
        cols: payload?.columnCount ?? 2,
        withHeaderRow: true,
      })
      .run();
  } else if (commandMap[command]) {
    handled = commandMap[command]();
  }

  syncTableState();
  if (handled) emit("change");
}

function insertText(text: string) {
  if (!text) return;
  editor.value
    ?.chain()
    .focus()
    .insertContent({ type: "text", text })
    .run();
  emit("change");
}

function replaceSelection(text: string) {
  const current = editor.value;
  if (!current) return;
  if (!text) {
    current.chain().focus().deleteSelection().run();
  } else {
    current
      .chain()
      .focus()
      .insertContent({ type: "text", text })
      .run();
  }
  emit("change");
}

function deleteSelection() {
  editor.value?.chain().focus().deleteSelection().run();
  emit("change");
}

function getSelectedText() {
  const current = editor.value;
  if (!current) return "";
  const { from, to } = current.state.selection;
  return current.state.doc.textBetween(from, to, "\n", "\ufffc");
}

// Builds a map from visible-text offsets (as produced by getVisibleText) to
// document positions, mirroring ProseMirror's
// `textBetween(0, size, "\n", "\ufffc")` exactly. textBetween emits a "\n"
// separator before every block (except the first) and one "\ufffc" per leaf
// node (e.g. an image). Reproducing that walk here — instead of re-deriving
// offsets with a separate heuristic — keeps find/replace indices in lockstep
// with the string the search actually runs against, so the "current" match is
// never skipped. Cached per document, since highlightMatches calls it per match.
let visibleTextCache: { doc: unknown; text: string; positions: number[] } | null =
  null;

function getVisibleTextMap() {
  const current = editor.value;
  if (!current) return { text: "", positions: [] as number[] };
  const doc = current.state.doc;
  if (visibleTextCache && visibleTextCache.doc === doc) return visibleTextCache;

  const chars: string[] = [];
  const positions: number[] = [];
  let first = true;
  doc.nodesBetween(0, doc.content.size, (node, pos) => {
    const nodeText = node.isText ? node.text || "" : node.isLeaf ? "\ufffc" : "";
    if (node.isBlock && ((node.isLeaf && nodeText) || node.isTextblock)) {
      // A block separator ("\n") is emitted before every block after the first.
      if (first) first = false;
      else {
        chars.push("\n");
        positions.push(pos);
      }
    }
    if (nodeText) {
      for (let i = 0; i < nodeText.length; i += 1) {
        chars.push(nodeText[i]);
        positions.push(pos + i);
      }
    }
    return true;
  });

  visibleTextCache = { doc, text: chars.join(""), positions };
  return visibleTextCache;
}

// Maps a visible-text offset (as produced by getVisibleText) back to a document
// position.
function visibleOffsetToPos(target: number) {
  const current = editor.value;
  const { positions } = getVisibleTextMap();
  if (!positions.length) return 1;
  if (target <= 0) return positions[0];
  if (target >= positions.length) {
    return current
      ? current.state.doc.content.size
      : positions[positions.length - 1] + 1;
  }
  return positions[target];
}

function getSelection(): EditorSelection | undefined {
  const current = editor.value;
  if (!current) return undefined;
  const { from, to } = current.state.selection;
  return [
    current.state.doc.textBetween(0, from, "\n", "\ufffc").length,
    current.state.doc.textBetween(0, to, "\n", "\ufffc").length,
  ];
}

function selectRange(start: number, end: number) {
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
  return getVisibleTextMap().text;
}

function highlightMatches(
  query: string,
  caseSensitive = false,
  activeOffset?: number,
) {
  const current = editor.value;
  if (!current) return;
  if (!query) {
    clearMatches();
    return;
  }

  const doc = current.state.doc;
  // Search against the same string the offsets are derived from (getVisibleText
  // delegates to this map), so an active offset from the parent always lands on
  // the right match.
  const text = getVisibleTextMap().text;
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
      // The active match gets an extra class so it stands out from the rest.
      // The editor selection isn't reliable here: the find box holds focus, so
      // browsers paint no (or a faint) selection for the match being jumped to.
      const isActive = activeOffset != null && offset === activeOffset;
      decorations.push(
        Decoration.inline(from, to, {
          class: isActive
            ? "mionote-find-match mionote-find-match-current"
            : "mionote-find-match",
        }),
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
  if (!editor.value) return null;
  return {
    bold: editor.value.isActive("bold"),
    italic: editor.value.isActive("italic"),
    strike: editor.value.isActive("strike"),
    underline: editor.value.isActive("underline"),
    inlineStyles: {
      color: editor.value.getAttributes("textStyle").color || "",
      "font-size": editor.value.getAttributes("fontSize").fontSize || "",
    },
  };
}

function applyInlineStyle(tag: string, style: Record<string, string> = {}) {
  if (!editor.value) return false;
  const chain = editor.value.chain().focus();
  if (tag === "strong") chain.toggleBold().run();
  else if (tag === "em") chain.toggleItalic().run();
  else if (tag === "strike") chain.toggleStrike().run();
  else if (tag === "u") chain.toggleUnderline().run();
  else if (tag === "mark")
    chain.setHighlight({ color: style["background-color"] }).run();
  else if (tag === "span" && style.color) chain.setColor(style.color).run();
  else if (tag === "span" && style["font-size"])
    chain.setFontSize(style["font-size"]).run();
  emit("change");
  return true;
}

function applyAlignment(alignment: string) {
  if (!editor.value) return false;
  editor.value.commands.setTextAlign?.(alignment);
  emit("change");
  return true;
}

function applyCopiedStyle(style: EditorSelectionStyle) {
  if (!editor.value) return false;
  const chain = editor.value.chain().focus();
  // Set marks to match the copied style exactly (set/unset rather than toggle,
  // so a target that already matches is not flipped off).
  if (style.bold) chain.setMark("bold");
  else chain.unsetMark("bold");
  if (style.italic) chain.setMark("italic");
  else chain.unsetMark("italic");
  if (style.underline) chain.setMark("underline");
  else chain.unsetMark("underline");
  if (style.strike) chain.setMark("strike");
  else chain.unsetMark("strike");
  const color = style.inlineStyles?.color;
  if (color) chain.setColor(color);
  else chain.unsetColor();
  const fontSize = style.inlineStyles?.["font-size"];
  if (fontSize) chain.setFontSize(fontSize);
  else chain.unsetFontSize();
  chain.run();
  emit("change");
  return Boolean(getSelectedText());
}

function exec(command: string, payload?: unknown) {
  if (command === "selectAll") {
    editor.value?.commands.selectAll();
  } else if (command === "addImage" && payload && typeof payload === "object") {
    const image = payload as { imageUrl?: string; altText?: string };
    editor.value
      ?.chain()
      .focus()
      .setImage({ src: image.imageUrl || "", alt: image.altText || "image" })
      .run();
    emit("change");
  } else {
    run(command, payload);
  }
}

function addImageFile(file: File) {
  if (!file) return;
  const callback = (url: string, altText = file.name) => {
    editor.value?.chain().focus().setImage({ src: url, alt: altText }).run();
    emit("change");
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
  const callback = (url: string, filename = file.name) => {
    editor.value
      ?.chain()
      .focus()
      .insertContent({
        type: "text",
        text: filename,
        marks: [{ type: "link", attrs: { href: url } }],
      })
      .run();
    emit("change");
  };
  if (props.addImageBlobHook) props.addImageBlobHook(file, callback);
}

function decorateBlocks() {
  const root = editor.value?.view.dom;
  if (!root) return;
  Array.from(root.children).forEach((child) => {
    const element = child as HTMLElement;
    // Only touch the DOM when something actually needs to change. Re-setting
    // an attribute on a selected block makes Chrome collapse the DOM selection,
    // which ProseMirror then reads back, dropping a NodeSelection (e.g. a
    // clicked image) as soon as any selection update fires.
    if (element.getAttribute("data-block") !== "true") {
      element.setAttribute("data-block", "true");
    }
    if (element.getAttribute("draggable") !== "true") {
      element.setAttribute("draggable", "true");
    }
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
    extensions: createNoteExtensions({ editor: true }),
    content: props.initialValue || "",
    autofocus: false,
    onUpdate: () => {
      emit("change");
      syncTableState();
      decorateBlocks();
    },
    onSelectionUpdate: () => {
      syncTableState();
    },
    // Clicking outside the editor (title, toolbar, sidebar) blurs it without
    // changing the selection, so hide the floating table controls manually.
    onBlur: () => {
      tableActive.value = false;
      tableControlsStyle.value = null;
    },
    onCreate: () =>
      nextTick(() => {
        syncTableState();
        decorateBlocks();
        emit("ready");
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
  clearMatches,
  deleteSelection,
  exec,
  focus,
  getHTML,
  getSelectedText,
  getSelection,
  getSelectionStyle,
  getVisibleText,
  highlightMatches,
  insertText,
  replaceSelection,
  selectRange,
  setHTML,
  setSelection,
});
</script>

<style>
.mionote-tiptap-editor {
  position: relative;
  display: block;
  color: rgb(var(--theme-text));
}

.mionote-tiptap-surface {
  min-height: 100%;
}

.mionote-tiptap-table-controls {
  position: absolute;
  z-index: 4;
  display: flex;
  align-items: center;
  gap: 0.125rem;
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

@media (max-width: 767px) {
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
}
</style>
