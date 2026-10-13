const tokenStorageKey = "token";

function getBasePath() {
  // This relies on the correctly formatted relative path in the <base> tag.
  return document.querySelector("base")?.getAttribute("href") ?? "/";
}

function getCookieString(token?: string) {
  const basePath = getBasePath();
  let cookie = `${tokenStorageKey}=${token}; Path=${basePath}; SameSite=Strict`;
  // `require()` on the backend also accepts auth via this cookie, which is how
  // `<img src="attachments/...">` requests authenticate (they can't send the
  // Bearer header). Without an explicit lifetime the cookie dies when the
  // browser session ends, so a persisted token would load APIs but show broken
  // images. Match the cookie lifetime to the token's own `exp` claim.
  const maxAge = token ? tokenMaxAge(token) : 0;
  if (maxAge > 0) cookie += `; Max-Age=${maxAge}`;
  return cookie;
}

function tokenMaxAge(token: string) {
  try {
    const base64 = token.split(".")[1].replace(/-/g, "+").replace(/_/g, "/");
    const padded = base64.padEnd(base64.length + ((4 - (base64.length % 4)) % 4), "=");
    const payload = JSON.parse(atob(padded));
    if (typeof payload.exp !== "number") return 0;
    return Math.max(0, payload.exp - Math.floor(Date.now() / 1000));
  } catch {
    return 0;
  }
}

export function storeToken(token, persist = false) {
  document.cookie = getCookieString(token);
  sessionStorage.setItem(tokenStorageKey, token);
  if (persist === true) {
    localStorage.setItem(tokenStorageKey, token);
  }
}

export function getStoredToken() {
  return sessionStorage.getItem(tokenStorageKey);
}

export function loadStoredToken() {
  const token = localStorage.getItem(tokenStorageKey);
  if (token != null) {
    storeToken(token, false);
  }
}

export function clearStoredToken() {
  sessionStorage.removeItem(tokenStorageKey);
  localStorage.removeItem(tokenStorageKey);
  document.cookie =
    getCookieString() + "; expires=Thu, 01 Jan 1970 00:00:00 GMT";
}

export function isCurrentTokenStored() {
  const localToken = localStorage.getItem(tokenStorageKey);
  if (localToken == null) {
    return false;
  }
  const sessionToken = sessionStorage.getItem(tokenStorageKey);
  return localToken === sessionToken;
}
