import { fetchAttachmentObjectUrl } from "../api";

// Attachments referenced by notes use a relative URL such as
// "attachments/foo.jpg" (see the backend AttachmentStore). `<img>` tags can't
// send the Authorization header, so these are resolved to authenticated blob
// URLs and cached so a node can be re-rendered (ProseMirror recreates image
// DOM on transactions) without re-fetching the bytes.
const objectUrls = new Map<string, Promise<string>>();

export function isAttachmentSrc(src: unknown): src is string {
  return typeof src === "string" && /(^|\/)(?:api\/)?attachments\//.test(src);
}

export function resolveAttachmentUrl(src: string): Promise<string> {
  let pending = objectUrls.get(src);
  if (!pending) {
    pending = fetchAttachmentObjectUrl(src).catch((error) => {
      // Don't cache failures so a later render can retry.
      objectUrls.delete(src);
      throw error;
    });
    objectUrls.set(src, pending);
  }
  return pending;
}
