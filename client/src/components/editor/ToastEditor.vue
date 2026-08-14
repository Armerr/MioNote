<template>
  <div ref="editorElement"></div>
</template>

<script setup lang="ts">
import Editor from "@toast-ui/editor";
import "@toast-ui/editor/dist/i18n/zh-cn";
import { DOMParser } from "prosemirror-model";
import { Plugin, PluginKey, TextSelection } from "prosemirror-state";
import { Decoration, DecorationSet } from "prosemirror-view";
import { onBeforeUnmount, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import baseOptions from "./baseOptions";
import type { EditorSelectionStyle } from "../../types";

const props = defineProps({
  initialValue: String,
  initialEditType: {
    type: String,
    default: "markdown",
  },
  addImageBlobHook: Function,
});

const emit = defineEmits(["change", "keydown"]);

const { locale } = useI18n();
const editorElement = ref();
let toastEditor;
let persistentTextColor = "";

const findHighlightKey = new PluginKey("mionoteFindHighlight");
// Toast UI plugin entries are factories: (eventEmitter) => PluginInfo for the
// top-level plugin and (eventEmitter) => Plugin for the per-mode PM plugins.
const findHighlightPmPlugin = () =>
  new Plugin({
    key: findHighlightKey,
    state: {
      init: () => DecorationSet.empty,
      apply(tr, value) {
        const meta = tr.getMeta(findHighlightKey);
        return meta != null ? meta : value.map(tr.mapping, tr.doc);
      },
    },
    props: {
      decorations(state) {
        return findHighlightKey.getState(state);
      },
    },
  });
const findHighlightPlugin = () => ({
  markdownPlugins: [findHighlightPmPlugin],
  wysiwygPlugins: [findHighlightPmPlugin],
});
const persistentTextColorPmPlugin = () =>
  new Plugin({
    appendTransaction(transactions, _, newState) {
      if (
        !persistentTextColor ||
        !transactions.some((transaction) => transaction.docChanged) ||
        !newState.selection.empty
      ) {
        return null;
      }

      const markType = newState.schema.marks.span;
      if (!markType) return null;

      const marks = newState.storedMarks ?? newState.selection.$from.marks();
      const hasPersistentColor = marks.some(
        (mark) =>
          mark.type === markType &&
          parseStyle(mark.attrs.htmlAttrs?.style).color === persistentTextColor,
      );
      if (hasPersistentColor) return null;

      return newState.tr.addStoredMark(
        markType.create({
          htmlAttrs: {
            style: serializeStyle({ color: persistentTextColor }),
          },
        }),
      );
    },
  });
const persistentTextColorPlugin = () => ({
  wysiwygPlugins: [persistentTextColorPmPlugin],
});

onMounted(() => {
  toastEditor = new Editor({
    ...baseOptions,
    plugins: [
      ...(baseOptions.plugins || []),
      findHighlightPlugin,
      persistentTextColorPlugin,
    ],
    el: editorElement.value,
    initialValue: props.initialValue,
    initialEditType: props.initialEditType,
    language: locale.value,
    events: {
      change: () => {
        emit("change");
      },
      keydown: (_, event) => {
        emit("keydown", event);
      },
    },
    hooks: props.addImageBlobHook
      ? { addImageBlobHook: props.addImageBlobHook }
      : {},
  });
  hideLegacyControls();
  requestAnimationFrame(hideLegacyControls);
});

function hideLegacyControls() {
  editorElement.value
    ?.querySelectorAll(
      ".toastui-editor-toolbar, .toastui-editor-md-tab-container, .toastui-editor-mode-switch",
    )
    .forEach((control) =>
      control.style.setProperty("display", "none", "important"),
    );
}

function getMarkdown() {
  return toastEditor.getMarkdown();
}

function getHTML() {
  return toastEditor?.getHTML() || "";
}

function setMarkdown(markdown) {
  toastEditor?.setMarkdown(markdown);
}

function focus() {
  toastEditor?.focus();
}

function isWysiwygMode() {
  return toastEditor.isWysiwygMode();
}

function exec(command, payload) {
  toastEditor?.exec(command, payload);
}

function getActiveEditor() {
  return toastEditor?.getCurrentModeEditor?.();
}

function insertText(text) {
  toastEditor?.insertText(text);
}

function deleteSelection() {
  toastEditor?.deleteSelection();
}

function getSelectedText() {
  return toastEditor?.getSelectedText() || "";
}

function replaceSelection(text) {
  toastEditor?.replaceSelection(text);
}

function setSelection(start, end) {
  toastEditor?.setSelection(start, end);
}

function getSelection() {
  return toastEditor?.getSelection?.();
}

// Maps a plain-text offset (as produced by doc.textBetween) back to a
// document position, accounting for the "\n" separators inserted between
// text blocks. Returns null when the offset falls outside the text range.
function textOffsetToDocPos(doc, target) {
  let found = null;
  let accumulated = 0;
  let firstBlock = true;
  doc.descendants((node, nodePos) => {
    if (found != null) return false;
    if (node.isText) {
      const next = accumulated + node.text.length;
      if (target >= accumulated && target <= next) {
        found = nodePos + (target - accumulated);
        return false;
      }
      accumulated = next;
      firstBlock = false;
    } else if (node.isTextblock) {
      if (!firstBlock) accumulated += 1;
    }
    return true;
  });
  return found;
}

// Highlights every occurrence of the query in the active editor with an
// inline decoration, or clears the decorations when the query is empty.
function highlightMatches(query, caseSensitive = false) {
  const editor = getActiveEditor();
  const view = editor?.view;
  if (!view) return;
  if (!query) {
    clearMatches();
    return;
  }

  const doc = view.state.doc;
  const text = doc.textBetween(0, doc.content.size, "\n", "\ufffc");
  const haystack = caseSensitive ? text : text.toLocaleLowerCase();
  const needle = caseSensitive ? query : query.toLocaleLowerCase();
  const decorations = [];
  let offset = haystack.indexOf(needle);
  let guard = 0;
  while (offset >= 0 && guard < 1000) {
    guard += 1;
    const from = textOffsetToDocPos(doc, offset);
    const to = textOffsetToDocPos(doc, offset + needle.length);
    if (from != null && to != null && to > from) {
      decorations.push(
        Decoration.inline(from, to, { class: "mionote-find-match" }),
      );
    }
    offset = haystack.indexOf(needle, offset + needle.length);
  }
  view.dispatch(
    view.state.tr.setMeta(
      findHighlightKey,
      DecorationSet.create(doc, decorations),
    ),
  );
}

function clearMatches() {
  const editor = getActiveEditor();
  const view = editor?.view;
  if (!view) return;
  view.dispatch(view.state.tr.setMeta(findHighlightKey, DecorationSet.empty));
}

// Returns the rendered (markdown-free) text of the active editor, as used by
// the page-local find mapping.
function getVisibleText() {
  const editor = getActiveEditor();
  const view = editor?.view;
  if (!view) return "";
  return view.state.doc.textBetween(
    0,
    view.state.doc.content.size,
    "\n",
    "\ufffc",
  );
}

// Selects the range given in rendered-text offsets through the ProseMirror
// state, so the selection survives editor redraws and focus changes.
function selectRange(from, to) {
  const editor = getActiveEditor();
  const view = editor?.view;
  if (!view) return;
  const doc = view.state.doc;
  const fromPos = textOffsetToDocPos(doc, from);
  const toPos = textOffsetToDocPos(doc, to);
  if (fromPos == null || toPos == null || toPos <= fromPos) return;
  view.dispatch(
    view.state.tr
      .setSelection(TextSelection.create(doc, fromPos, toPos))
      .scrollIntoView(),
  );
  toastEditor?.focus();
}

function changeMode(mode) {
  toastEditor?.changeMode(mode);
  requestAnimationFrame(hideLegacyControls);
}

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/\"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

function styleAttributes(style) {
  const value = serializeStyle(style);
  return value ? ` style="${escapeHtml(value)}"` : "";
}

function serializeStyle(style) {
  return Object.entries(style || {})
    .filter(([, cssValue]) => cssValue)
    .map(([property, cssValue]) => `${property}: ${cssValue}`)
    .join("; ");
}

function parseStyle(style = ""): Record<string, string> {
  return style
    .split(";")
    .reduce<Record<string, string>>((styles, declaration) => {
      const separator = declaration.indexOf(":");
      if (separator === -1) return styles;

      const property = declaration.slice(0, separator).trim();
      const value = declaration.slice(separator + 1).trim();
      if (property && value) styles[property] = value;
      return styles;
    }, {});
}

function wrapStyledText(
  content: string,
  copiedStyle: EditorSelectionStyle = {},
) {
  if (!content) return "";

  let result = content;
  if (
    copiedStyle.inlineStyles &&
    Object.values(copiedStyle.inlineStyles).some(Boolean)
  ) {
    result = `<span${styleAttributes(copiedStyle.inlineStyles)}>${result}</span>`;
  }
  if (copiedStyle.highlight) {
    result = `<mark${styleAttributes({ "background-color": copiedStyle.highlight })}>${result}</mark>`;
  }
  if (copiedStyle.underline) {
    result = `<u>${result}</u>`;
  }
  if (copiedStyle.strike) {
    result = `<del>${result}</del>`;
  }
  if (copiedStyle.italic) {
    result = `<em>${result}</em>`;
  }
  if (copiedStyle.bold) {
    result = `<strong>${result}</strong>`;
  }

  return result;
}

function markdownPositionToOffset(markdown, position) {
  if (!Array.isArray(position) || position.length !== 2) return null;

  const [line, column] = position;
  if (!Number.isFinite(line) || !Number.isFinite(column)) return null;

  const lines = markdown.split("\n");
  const safeLine = Math.min(Math.max(1, line), lines.length);
  const lineIndex = safeLine - 1;
  const offset = lines
    .slice(0, lineIndex)
    .reduce((total, value) => total + value.length + 1, 0);
  const safeColumn = Math.min(Math.max(1, column), lines[lineIndex].length + 1);
  return offset + safeColumn - 1;
}

function applyInlineStyle(tag, style: Record<string, string> = {}) {
  if (!toastEditor) return false;

  const attributes = styleAttributes(style);
  const openTag = `<${tag}${attributes}>`;
  const closeTag = `</${tag}>`;

  if (toastEditor.isMarkdownMode()) {
    const selection = toastEditor.getSelectedText();
    if (!selection) return false;

    toastEditor.replaceSelection(`${openTag}${selection}${closeTag}`);
    return true;
  }

  const editor = getActiveEditor();
  const { state, dispatch } = editor?.view || {};
  const { from, to } = state?.selection || {};
  const markType = state?.schema?.marks?.[tag];
  if (!markType || !Number.isFinite(from) || !Number.isFinite(to)) return false;

  const mark = markType.create({
    htmlAttrs: { style: serializeStyle(style) },
  });
  if (tag === "span" && style.color) persistentTextColor = style.color;
  let transaction = state.tr;
  if (from !== to) transaction = transaction.addMark(from, to, mark);

  // Stored marks define the style of the next inserted character. Keeping the
  // cursor at the selection end makes a color change apply to subsequent input.
  transaction = transaction
    .setSelection(TextSelection.create(transaction.doc, to))
    .addStoredMark(mark);
  editor.view.dispatch(transaction.scrollIntoView());
  toastEditor.focus();
  return true;
}

function applyAlignment(alignment) {
  if (!toastEditor) return false;

  const editor = getActiveEditor();

  if (toastEditor.isMarkdownMode()) {
    const markdown = toastEditor.getMarkdown();
    const [startPos, endPos] = editor?.getSelection?.() || [];
    const startOffset = markdownPositionToOffset(markdown, startPos);
    const endOffset = markdownPositionToOffset(markdown, endPos);
    if (startOffset == null || endOffset == null) return false;

    const blockStart =
      markdown.lastIndexOf("\n", Math.max(0, startOffset - 1)) + 1;
    const blockEndAnchor = Math.max(startOffset, endOffset) - 1;
    const blockEndLineBreak = markdown.indexOf(
      "\n",
      Math.max(0, blockEndAnchor),
    );
    const blockEnd =
      blockEndLineBreak === -1 ? markdown.length : blockEndLineBreak;
    const line = markdown.slice(blockStart, blockEnd);
    if (!line) return false;

    const openTag = `<span style="display: block; text-align: ${alignment}">`;
    const content = `${openTag}${line}</span>`;
    toastEditor.setMarkdown(
      `${markdown.slice(0, blockStart)}${content}${markdown.slice(blockEnd)}`,
      false,
    );
    return true;
  }

  const { state, dispatch } = editor?.view || {};
  const { $from } = state?.selection || {};
  if (!$from) return false;

  const markType = state?.schema?.marks?.span;
  if (!markType) return false;

  for (let depth = $from.depth; depth > 0; depth -= 1) {
    const node = $from.node(depth);
    if (!node.isTextblock) continue;

    const start = $from.start(depth);
    const end = $from.end(depth);
    if (start === end) return false;

    editor.view.dispatch(
      state.tr.addMark(
        start,
        end,
        markType.create({
          htmlAttrs: { style: `display: block; text-align: ${alignment}` },
        }),
      ),
    );
    toastEditor.focus();
    return true;
  }

  toastEditor.focus();
  return false;
}

function getSelectionStyle() {
  if (!toastEditor?.isWysiwygMode()) return null;

  const editor = getActiveEditor();
  const { state } = editor?.view || {};
  const marks = state?.selection?.$from?.marks?.() || [];
  const styles = marks
    .filter((mark) => mark.type.name === "span")
    .reduce(
      (value, mark) => ({
        ...value,
        ...parseStyle(mark.attrs.htmlAttrs?.style),
      }),
      {},
    );
  const highlightStyles = marks
    .filter((mark) => mark.type.name === "mark")
    .reduce(
      (value, mark) => ({
        ...value,
        ...parseStyle(mark.attrs.htmlAttrs?.style),
      }),
      {},
    );

  return {
    inlineStyles: {
      color: styles.color || "",
      "font-size": styles["font-size"] || "",
    },
    bold: marks.some((mark) => mark.type.name === "strong"),
    highlight: highlightStyles["background-color"] || "",
    italic: marks.some((mark) => mark.type.name === "emph"),
    strike: marks.some((mark) => mark.type.name === "strike"),
    underline: marks.some((mark) => mark.type.name === "u"),
  };
}

function applyCopiedStyle(copiedStyle) {
  const selection = toastEditor?.getSelectedText?.() || "";
  if (!selection) return false;

  const wrapped = wrapStyledText(selection, copiedStyle);
  if (!wrapped) return false;

  if (toastEditor.isMarkdownMode()) {
    toastEditor.replaceSelection(wrapped);
    return true;
  }

  return insertHtml(wrapped);
}

function insertHtml(html) {
  if (!toastEditor || !html) return false;

  if (toastEditor.isMarkdownMode()) {
    toastEditor.insertText(html);
    return true;
  }

  const editor = getActiveEditor();
  const { state, dispatch } = editor?.view || {};
  if (!state || !dispatch) return false;

  const template = document.createElement("template");
  template.innerHTML = html;
  const slice = DOMParser.fromSchema(state.schema).parseSlice(template.content);
  editor.view.dispatch(state.tr.replaceSelection(slice).scrollIntoView());
  toastEditor.focus();
  return true;
}

function addImageFile(file) {
  if (!file) return;

  const callback = (url: string, altText = file.name) => {
    toastEditor?.exec("addImage", {
      imageUrl: url,
      altText: altText || file.name,
    });
  };

  if (props.addImageBlobHook) {
    props.addImageBlobHook(file, callback);
    return;
  }

  const reader = new FileReader();
  reader.onload = ({ target }) =>
    callback(typeof target?.result === "string" ? target.result : "");
  reader.readAsDataURL(file);
}

function addMediaFile(file) {
  if (!file) return;
  if (file.type.startsWith("image/")) {
    addImageFile(file);
    return;
  }

  const callback = (url) => {
    insertHtml(
      `<video controls src="${escapeHtml(url)}">${escapeHtml(file.name)}</video>`,
    );
  };

  if (props.addImageBlobHook) {
    props.addImageBlobHook(file, callback);
    return;
  }

  const reader = new FileReader();
  reader.onload = ({ target }) => callback(target?.result);
  reader.readAsDataURL(file);
}

function addAttachmentFile(file) {
  if (!file) return;

  const callback = (url, filename) => {
    const label = filename || file.name;
    insertHtml(
      `<a href="${escapeHtml(url)}" target="_blank" rel="noopener noreferrer">${escapeHtml(label)}</a>`,
    );
  };

  if (props.addImageBlobHook) {
    props.addImageBlobHook(file, callback);
    return;
  }

  const reader = new FileReader();
  reader.onload = ({ target }) => callback(target?.result, file.name);
  reader.readAsDataURL(file);
}

onBeforeUnmount(() => {
  toastEditor?.destroy();
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
  getSelection,
  getSelectedText,
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
@import "@toast-ui/editor/dist/toastui-editor.css";
@import "prismjs/themes/prism.css";
@import "@toast-ui/editor-plugin-code-syntax-highlight/dist/toastui-editor-plugin-code-syntax-highlight.css";
@import "./toastui-editor-overrides.scss";
</style>
