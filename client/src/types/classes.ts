import { i18n } from "../i18n";

export interface NoteData {
  title?: string;
  lastModified?: number;
  content?: string | null;
}

export interface SearchResultData extends NoteData {
  preview?: string | null;
  score?: number | null;
  titleHighlights?: string | null;
  contentHighlights?: string | null;
  tagMatches?: string[] | null;
}

export class Note {
  title: string;
  lastModified: number;
  content: string;

  constructor(note: NoteData = {}) {
    this.title = note.title ?? "";
    this.lastModified = note.lastModified ?? 0;
    this.content = note.content ?? "";
  }

  get lastModifiedAsDate() {
    return new Date(this.lastModified * 1000);
  }

  get lastModifiedAsString() {
    return this.lastModifiedAsDate.toLocaleString(i18n.global.locale.value);
  }
}

export class SearchResult extends Note {
  preview: string;
  score: number;
  titleHighlights: string;
  contentHighlights: string;
  tagMatches: string[];

  constructor(searchResult: SearchResultData = {}) {
    super(searchResult);
    this.preview = searchResult.preview ?? "";
    this.score = searchResult.score ?? 0;
    this.titleHighlights = searchResult.titleHighlights ?? "";
    this.contentHighlights = searchResult.contentHighlights ?? "";
    this.tagMatches = searchResult.tagMatches ?? [];
  }

  get titleHighlightsOrTitle() {
    return this.titleHighlights ? this.titleHighlights : this.title;
  }

  get includesHighlights() {
    if (
      this.titleHighlights ||
      this.contentHighlights ||
      this.tagMatches.length
    ) {
      return true;
    } else {
      return false;
    }
  }
}
