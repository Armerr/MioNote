import * as constants from "../utils/constants";

import { Note, SearchResult } from "../types/classes";

import axios from "axios";
import { clearStoredToken, getStoredToken } from "./tokenStorage";
import { getToastOptions } from "../utils/helpers";
import { i18n } from "../i18n";
import router from "../router";
import type { NoteData, SearchResultData } from "../types/classes";
import type { AppConfig, CurrentUser, ToastController } from "../types";

const api = axios.create();

api.interceptors.request.use(
  // If the request is not for the token endpoint, add the token to the headers.
  function (config) {
    if (!["api/token", "api/register"].includes(config.url)) {
      const token = getStoredToken();
      if (token) {
        config.headers.Authorization = `Bearer ${token}`;
      }
    }
    return config;
  },
  function (error) {
    return Promise.reject(error);
  },
);

export function apiErrorHandler(error: any, toast?: ToastController) {
  if (error.response?.status === 401) {
    clearStoredToken();
    const redirectPath = router.currentRoute.value.fullPath;
    router.push({
      name: "login",
      query: { [constants.params.redirect]: redirectPath },
    });
  } else {
    console.error(error);
    if (toast) {
      toast.add(
        getToastOptions(
          i18n.global.t("common.unknownError"),
          i18n.global.t("common.error"),
          "error",
        ),
      );
    }
  }
}

export async function getConfig(): Promise<AppConfig> {
  try {
    const response = await api.get("api/config");
    return response.data as AppConfig;
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function getToken(
  username: string,
  password: string,
): Promise<string> {
  try {
    const response = await api.post("api/token", {
      username: username,
      password: password,
    });
    return response.data.access_token;
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function registerAccount(
  username: string,
  password: string,
): Promise<string> {
  try {
    const response = await api.post("api/register", { username, password });
    return response.data.access_token;
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function getCurrentUser(): Promise<CurrentUser> {
  try {
    const response = await api.get("api/users/me");
    return response.data as CurrentUser;
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function authCheck() {
  try {
    const response = await api.get("api/auth-check");
    return response.data;
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function getNotes(
  term = "*",
  sort?: string,
  order?: string,
  limit?: number,
): Promise<SearchResult[]> {
  try {
    const response = await api.get("api/search", {
      params: {
        term: term,
        sort: sort,
        order: order,
        limit: limit,
      },
    });
    return (response.data as SearchResultData[]).map(
      (note) => new SearchResult(note),
    );
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function createNote(
  title: string,
  content: string,
): Promise<Note> {
  try {
    const response = await api.post("api/notes", {
      title: title,
      content: content,
    });
    return new Note(response.data as NoteData);
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function getNote(title: string): Promise<Note> {
  try {
    const response = await api.get(`api/notes/${encodeURIComponent(title)}`);
    return new Note(response.data as NoteData);
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function updateNote(
  title: string,
  newTitle: string,
  newContent: string,
): Promise<Note> {
  try {
    const response = await api.patch(`api/notes/${encodeURIComponent(title)}`, {
      newTitle: newTitle,
      newContent: newContent,
    });
    return new Note(response.data as NoteData);
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function deleteNote(title: string): Promise<void> {
  try {
    await api.delete(`api/notes/${encodeURIComponent(title)}`);
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function getTags(): Promise<string[]> {
  try {
    const response = await api.get("api/tags");
    return response.data as string[];
  } catch (response) {
    return Promise.reject(response);
  }
}

export async function createAttachment(
  file: File,
): Promise<{ filename: string; url: string }> {
  try {
    const formData = new FormData();
    formData.append("file", file);
    const response = await api.post("api/attachments", formData, {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    });
    return response.data as { filename: string; url: string };
  } catch (response) {
    return Promise.reject(response);
  }
}
