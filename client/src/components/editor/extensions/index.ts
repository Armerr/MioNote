import { Extension, type Extensions } from "@tiptap/core";
import StarterKit from "@tiptap/starter-kit";
import Link from "@tiptap/extension-link";
import Image from "@tiptap/extension-image";
import { TableKit } from "@tiptap/extension-table";
import TaskList from "@tiptap/extension-task-list";
import TaskItem from "@tiptap/extension-task-item";
import Highlight from "@tiptap/extension-highlight";
import Underline from "@tiptap/extension-underline";
import TextAlign from "@tiptap/extension-text-align";
import Placeholder from "@tiptap/extension-placeholder";
import UniqueID from "@tiptap/extension-unique-id";
import { Plugin, PluginKey } from "@tiptap/pm/state";
import { DecorationSet } from "@tiptap/pm/view";
import { VueNodeViewRenderer } from "@tiptap/vue-3";

import NoteImageView from "../NoteImageView.vue";
import { FontSize, TextStyle } from "./richTextMarks";

/**
 * Image with a Vue node view so attachments are loaded through the
 * authenticated API client (a blob URL) instead of a bare `<img src>`, which
 * could only authenticate via the fragile `token` cookie and showed as a broken
 * image otherwise. Non-attachment images (data URLs, external http) render
 * directly.
 */
const NoteImage = Image.extend({
  addNodeView() {
    return VueNodeViewRenderer(NoteImageView);
  },
});

/**
 * Node types that receive a stable `id` (used by the editor's block drag
 * handles). Only registered for the editable editor.
 */
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

export const findHighlightKey = new PluginKey<DecorationSet>(
  "mionoteFindHighlight",
);

/**
 * Decorates find/replace matches. Shared by the editor so the same plugin key
 * can be used to dispatch and clear decorations.
 */
export const FindHighlight = Extension.create({
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

/**
 * The rich-text schema shared by the editable editor and the read-only viewer.
 * Editor-only extensions (placeholder, block ids, find highlighting) are added
 * when `editor` is true so the viewer stays minimal and inert.
 */
export function createNoteExtensions({
  editor = true,
}: { editor?: boolean } = {}): Extensions {
  const extensions: Extensions = [
    StarterKit.configure({ link: false, underline: false }),
    Link.configure({ openOnClick: false, autolink: true }),
    NoteImage.configure({ inline: false, allowBase64: true }),
    TableKit.configure({ table: { resizable: true } }),
    TaskList,
    TaskItem.configure({ nested: true }),
    Highlight.configure({ multicolor: true }),
    Underline,
    TextStyle,
    FontSize,
    TextAlign.configure({ types: ["heading", "paragraph"] }),
  ];

  if (editor) {
    extensions.push(
      Placeholder.configure({ placeholder: "开始记录..." }),
      UniqueID.configure({ types: topLevelTypes }),
      FindHighlight,
    );
  }

  return extensions;
}
