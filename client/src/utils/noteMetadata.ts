function key(userId, title, property) {
  return `mionote:note:${userId || "anonymous"}:${title}:${property}`;
}

export function getNoteMetadata(userId, title, property, fallback = "") {
  return localStorage.getItem(key(userId, title, property)) ?? fallback;
}

export function setNoteMetadata(userId, title, property, value) {
  localStorage.setItem(key(userId, title, property), String(value));
}

export function removeNoteMetadata(userId, title, property) {
  localStorage.removeItem(key(userId, title, property));
}
