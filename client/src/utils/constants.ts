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

// The sole untitled note is stored under this canonical filename.
export const defaultNoteTitle = "新建笔记";
