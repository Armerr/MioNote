import type { Component } from "vue";

export interface AppConfig {
  authType: string;
  registrationOpen: boolean;
}

export interface CurrentUser {
  id: number;
  username: string;
}

export interface ToastOptions {
  summary?: string;
  detail?: string;
  severity?: ToastSeverity;
  closable?: boolean;
  life?: number;
}

export interface Toast extends Required<Pick<ToastOptions, "severity">> {
  id: number;
  summary?: string;
  detail?: string;
}

export type ToastSeverity = "info" | "success" | "error";

export interface ToastController {
  add: (options?: ToastOptions) => Toast;
  remove: (id: number) => void;
}

export interface MenuItem {
  id?: string;
  label?: string;
  icon?: Component;
  trailingIcon?: Component;
  separator?: boolean;
  disabled?: boolean;
  danger?: boolean;
  active?: boolean;
  mobileOnly?: boolean;
  chevron?: boolean;
  keyboardShortcut?: string;
  command?: () => void;
  children?: MenuItem[];
}

export interface SegmentedControlItem {
  value: string;
  label: string;
  icon?: Component;
  disabled?: boolean;
  title?: string;
  ariaLabel?: string;
}

export type MenuAlignment = "start" | "center" | "end";

export type ButtonVariant =
  | "default"
  | "secondary"
  | "outline"
  | "ghost"
  | "destructive"
  | "success"
  | "link";

export type ButtonSize =
  "default" | "sm" | "lg" | "icon" | "icon-sm" | "icon-lg";

export type ButtonType = "button" | "submit" | "reset";

export type EditorMode = "markdown" | "wysiwyg";
export type EditorSelection = [number, number];

export interface NoteHistoryEntry {
  title: string;
  content: string;
  savedAt: number;
}

export interface EditorSelectionStyle {
  inlineStyles?: Record<string, string>;
  highlight?: string;
  underline?: boolean;
  strike?: boolean;
  italic?: boolean;
  bold?: boolean;
}

export interface ToastEditorHandle {
  addAttachmentFile: (file: File) => void;
  addImageFile: (file: File) => void;
  addMediaFile: (file: File) => void;
  applyAlignment: (alignment: string) => boolean;
  applyCopiedStyle: (style: EditorSelectionStyle) => boolean;
  applyInlineStyle: (tag: string, style?: Record<string, string>) => boolean;
  changeMode: (mode: EditorMode) => void;
  clearMatches: () => void;
  deleteSelection: () => void;
  exec: (command: string, payload?: unknown) => void;
  focus: () => void;
  getHTML: () => string;
  getMarkdown: () => string;
  getSelectedText: () => string;
  getSelection: () => EditorSelection | undefined;
  getSelectionStyle: () => EditorSelectionStyle | null;
  getVisibleText: () => string;
  highlightMatches: (query: string, caseSensitive?: boolean) => void;
  insertText: (text: string) => void;
  isWysiwygMode: () => boolean;
  replaceSelection: (text: string) => void;
  selectRange: (from: number, to: number) => void;
  setMarkdown: (markdown: string) => void;
  setSelection: (start: number, end: number) => void;
}
