import { params, searchSortOptions } from "../../utils/constants";

import router from "../../router";

/*
 * Sourced from toast-ui. Their autolink options are
 * either override their built in functionality or
 * use their built in functionality. We'd like to have
 * both so this is the source of their parsers.
 */
const DOMAIN = "(?:[w-]+.)*[A-Za-z0-9-]+.[A-Za-z0-9-]+";
const PATH = "[^<\\s]*[^<?!.,:*_?~\\s]";
const EMAIL = "[\\w.+-]+@(?:[\\w-]+\\.)+[\\w-]+";

interface Autolink {
  text: string;
  range: [number, number];
  url: string;
}

function trimUnmatchedTrailingParens(source: string) {
  const trailingParen = /\)+$/.exec(source);
  if (trailingParen) {
    let count = 0;
    for (const ch of source) {
      if (ch === "(") {
        if (count < 0) {
          count = 1;
        } else {
          count += 1;
        }
      } else if (ch === ")") {
        count -= 1;
      }
    }

    if (count < 0) {
      const trimCount = Math.min(-count, trailingParen[0].length);
      return source.substring(0, source.length - trimCount);
    }
  }
  return source;
}

function trimTrailingEntity(source: string) {
  return source.replace(/&[A-Za-z0-9]+;$/, "");
}
export function parseEmailLink(source: string): Autolink[] {
  const reEmailLink = new RegExp(EMAIL, "g");
  const result: Autolink[] = [];
  let m;
  while ((m = reEmailLink.exec(source))) {
    const text = m[0];
    if (!/[_-]+$/.test(text)) {
      result.push({
        text,
        range: [m.index, m.index + text.length - 1],
        url: `mailto:${text}`,
      });
    }
  }

  return result;
}

export function parseUrlLink(source: string): Autolink[] {
  const reWwwAutolink = new RegExp(`(www|https?://)\.${DOMAIN}${PATH}`, "g");
  const result: Autolink[] = [];
  let m;

  while ((m = reWwwAutolink.exec(source))) {
    const text = trimTrailingEntity(trimUnmatchedTrailingParens(m[0]));
    const scheme = m[1] === "www" ? "http://" : "";
    result.push({
      text,
      range: [m.index, m.index + text.length - 1],
      url: `${scheme}${text}`,
    });
  }

  return result;
}
// end of raw toast-ui source

function parseWikiLink(source: string): Autolink[] {
  const matched: RegExpMatchArray[] = Array.from(
    source.matchAll(/\[\[\s*(\S(?:[^\[\]]*?\S)?)\s*\]\]/g),
  );
  if (matched.length) {
    return matched.map((match) => {
      const text = match[1];
      const index = match.index ?? 0;
      return {
        text,
        range: [index, index + match[0].length - 1],
        url: `${router.resolve({ name: "note", params: { title: text.trim() } }).href}`,
      };
    });
  }

  return [];
}

function parseTagLink(source: string): Autolink[] {
  const matched: RegExpMatchArray[] = Array.from(
    source.matchAll(/(?:^|\s)(#[\p{L}\p{N}_-]+)(?=\s|$)/gu),
  );
  if (matched.length) {
    return matched.map((match) => {
      const text = match[1];
      const index = match.index ?? 0;
      return {
        text,
        range: [
          index + match[0].indexOf(text),
          index + match[0].indexOf(text) + text.length - 1,
        ],
        url: `${
          router.resolve({
            name: "search",
            query: {
              [params.searchTerm]: text,
              [params.sortBy]: searchSortOptions.title,
            },
          }).href
        }`,
      };
    });
  }

  return [];
}

function extendedAutolinks(source: string): Autolink[] {
  return [
    ...parseUrlLink(source),
    ...parseEmailLink(source),
    ...parseWikiLink(source),
    ...parseTagLink(source),
  ].sort((a, b) => a.range[0] - b.range[0]);
}

export default extendedAutolinks;
