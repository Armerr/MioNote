// Params
export const params = {
  searchTerm: "term",
  redirect: "redirect",
  showHighlights: "showHighlights",
  sortBy: "sortBy",
};

export const searchSortOptions = {
  score: 0,
  title: 1,
  lastModified: 2,
};

export const authTypes = {
  none: "none",
  readOnly: "read_only",
  password: "password",
  totp: "totp",
};

// Untitled notes use the canonical name when available, or a hidden unique
// filename when another untitled note already occupies it.
export const defaultNoteTitle = "新建笔记";
export const untitledTitlePrefix = ".mionote-untitled-";

export function isUntitledNoteTitle(title: string) {
  return title === defaultNoteTitle || title.startsWith(untitledTitlePrefix);
}
