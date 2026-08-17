use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use regex::Regex;
use tokio::fs;
use tokio::sync::Mutex;

use crate::{
    error::{AppError, AppResult},
    models::{Note, NoteCreate, NoteUpdate, SearchResult},
    search::{extract_tags, search_content},
    SearchQuery,
};

const DEFAULT_NOTE_TITLE: &str = "新建笔记";
const UNTITLED_TITLE_PREFIX: &str = ".mionote-untitled-";

pub struct NoteStore {
    storage_path: PathBuf,
    write_lock: Mutex<()>,
}

impl NoteStore {
    pub async fn new(storage_path: PathBuf) -> AppResult<Self> {
        fs::create_dir_all(&storage_path)
            .await
            .map_err(AppError::internal)?;
        Ok(Self {
            storage_path,
            write_lock: Mutex::new(()),
        })
    }

    pub async fn index_html(&self, path_prefix: &str) -> AppResult<String> {
        let mut html = fs::read_to_string("client/dist/index.html")
            .await
            .map_err(AppError::internal)?;
        let base = if path_prefix.is_empty() {
            "/".to_string()
        } else {
            format!("{path_prefix}/")
        };
        let base_pattern =
            Regex::new(r#"(?i)(<base\s+href=")[^"]*(")"#).map_err(AppError::internal)?;
        html = base_pattern
            .replace(&html, |captures: &regex::Captures| {
                format!("{}{}{}", &captures[1], base, &captures[2])
            })
            .to_string();
        Ok(html)
    }

    pub async fn create(&self, user_id: i64, data: NoteCreate) -> AppResult<Note> {
        let requested_title = validate_title(&data.title)?;
        let _guard = self.write_lock.lock().await;
        self.ensure_user_storage(user_id).await?;
        if requested_title == DEFAULT_NOTE_TITLE {
            if let Some(title) = self.find_empty_untitled_title(user_id).await? {
                let content = fs::read_to_string(self.path_for(user_id, &title))
                    .await
                    .map_err(AppError::internal)?;
                return self.note_from_file(user_id, &title, content).await;
            }
        }
        let title = self.title_for_create(user_id, &requested_title).await?;
        let path = self.path_for(user_id, &title);
        let content = data.content.unwrap_or_default();
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        let mut file = options.open(&path).await.map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                AppError::Conflict(
                    "Cannot create note. A note with the same title already exists.".to_string(),
                )
            } else {
                AppError::internal(error)
            }
        })?;
        use tokio::io::AsyncWriteExt;
        file.write_all(content.as_bytes())
            .await
            .map_err(AppError::internal)?;
        self.note_from_file(user_id, &title, content).await
    }

    pub async fn get(&self, user_id: i64, title: &str) -> AppResult<Note> {
        let title = validate_title(title)?;
        let content = fs::read_to_string(self.path_for(user_id, &title))
            .await
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    AppError::NotFound("The specified note cannot be found.".to_string())
                } else {
                    AppError::internal(error)
                }
            })?;
        self.note_from_file(user_id, &title, content).await
    }

    pub async fn update(&self, user_id: i64, old_title: &str, data: NoteUpdate) -> AppResult<Note> {
        let old_title = validate_title(old_title)?;
        let mut new_title = match data.new_title {
            Some(value) => validate_title(&value)?,
            None => old_title.clone(),
        };
        let _guard = self.write_lock.lock().await;
        let old_path = self.path_for(user_id, &old_title);
        let mut new_path = self.path_for(user_id, &new_title);
        if new_title == DEFAULT_NOTE_TITLE
            && old_title != DEFAULT_NOTE_TITLE
            && fs::read_to_string(&old_path)
                .await
                .map_err(AppError::internal)?
                .is_empty()
            && self.find_empty_untitled_title(user_id).await?.is_some()
        {
            return Err(AppError::Conflict(
                "Cannot create a second untitled empty note.".to_string(),
            ));
        }
        if old_path != new_path
            && fs::try_exists(&new_path)
                .await
                .map_err(AppError::internal)?
        {
            if new_title == DEFAULT_NOTE_TITLE
                && !self
                    .would_create_duplicate_empty_note(&old_path, &new_path)
                    .await?
            {
                new_title = self.allocate_untitled_title(user_id).await?;
                new_path = self.path_for(user_id, &new_title);
            } else {
                return Err(AppError::Conflict(
                    "Cannot create note. A note with the same title already exists.".to_string(),
                ));
            }
        }
        if old_path != new_path {
            fs::rename(&old_path, &new_path).await.map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    AppError::NotFound("The specified note cannot be found.".to_string())
                } else {
                    AppError::internal(error)
                }
            })?;
        }
        let content = match data.new_content {
            Some(content) => {
                write_file(&new_path, &content).await?;
                content
            }
            None => fs::read_to_string(&new_path)
                .await
                .map_err(AppError::internal)?,
        };
        self.note_from_file(user_id, &new_title, content).await
    }

    pub async fn delete(&self, user_id: i64, title: &str) -> AppResult<()> {
        let title = validate_title(title)?;
        let _guard = self.write_lock.lock().await;
        fs::remove_file(self.path_for(user_id, &title))
            .await
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    AppError::NotFound("The specified note cannot be found.".to_string())
                } else {
                    AppError::internal(error)
                }
            })
    }

    pub async fn search(&self, user_id: i64, query: SearchQuery) -> AppResult<Vec<SearchResult>> {
        let user_path = self.user_storage_path(user_id);
        if !fs::try_exists(&user_path)
            .await
            .map_err(AppError::internal)?
        {
            return Ok(Vec::new());
        }
        let mut entries = fs::read_dir(user_path).await.map_err(AppError::internal)?;
        let mut results = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(AppError::internal)? {
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("md") {
                continue;
            }
            let Some(title) = path
                .file_stem()
                .and_then(|value| value.to_str())
                .map(str::to_string)
            else {
                continue;
            };
            let content = fs::read_to_string(&path)
                .await
                .map_err(AppError::internal)?;
            let modified = entry
                .metadata()
                .await
                .map_err(AppError::internal)?
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH);
            let last_modified = modified
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64();
            if let Some(result) = search_content(&title, &content, last_modified, &query.term) {
                results.push(result);
            }
        }
        let descending = query.order.as_deref().unwrap_or("desc") == "desc";
        match query.sort.as_deref().unwrap_or("score") {
            "title" => results.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase())),
            "lastModified" | "last_modified" => {
                results.sort_by(|a, b| a.last_modified.total_cmp(&b.last_modified))
            }
            _ => results.sort_by(|a, b| {
                a.score
                    .unwrap_or_default()
                    .total_cmp(&b.score.unwrap_or_default())
            }),
        }
        if descending {
            results.reverse();
        }
        if let Some(limit) = query.limit {
            results.truncate(limit.min(1000));
        }
        Ok(results)
    }

    pub async fn tags(&self, user_id: i64) -> AppResult<Vec<String>> {
        let user_path = self.user_storage_path(user_id);
        if !fs::try_exists(&user_path)
            .await
            .map_err(AppError::internal)?
        {
            return Ok(Vec::new());
        }
        let mut entries = fs::read_dir(user_path).await.map_err(AppError::internal)?;
        let mut tags = std::collections::BTreeSet::new();
        while let Some(entry) = entries.next_entry().await.map_err(AppError::internal)? {
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("md") {
                continue;
            }
            let content = fs::read_to_string(path).await.map_err(AppError::internal)?;
            tags.extend(extract_tags(&content));
        }
        Ok(tags.into_iter().collect())
    }

    fn user_storage_path(&self, user_id: i64) -> PathBuf {
        self.storage_path.join("users").join(user_id.to_string())
    }

    async fn ensure_user_storage(&self, user_id: i64) -> AppResult<()> {
        fs::create_dir_all(self.user_storage_path(user_id))
            .await
            .map_err(AppError::internal)
    }

    async fn title_for_create(&self, user_id: i64, requested_title: &str) -> AppResult<String> {
        let path = self.path_for(user_id, requested_title);
        if requested_title != DEFAULT_NOTE_TITLE
            || !fs::try_exists(&path).await.map_err(AppError::internal)?
        {
            return Ok(requested_title.to_string());
        }

        let content = fs::read_to_string(path).await.map_err(AppError::internal)?;
        if content.is_empty() {
            Ok(requested_title.to_string())
        } else {
            self.allocate_untitled_title(user_id).await
        }
    }

    async fn would_create_duplicate_empty_note(
        &self,
        old_path: &PathBuf,
        existing_path: &PathBuf,
    ) -> AppResult<bool> {
        let old_content = fs::read_to_string(old_path)
            .await
            .map_err(AppError::internal)?;
        let existing_content = fs::read_to_string(existing_path)
            .await
            .map_err(AppError::internal)?;
        Ok(old_content.is_empty() && existing_content.is_empty())
    }

    async fn find_empty_untitled_title(&self, user_id: i64) -> AppResult<Option<String>> {
        let mut entries = fs::read_dir(self.user_storage_path(user_id))
            .await
            .map_err(AppError::internal)?;
        while let Some(entry) = entries.next_entry().await.map_err(AppError::internal)? {
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("md") {
                continue;
            }
            let Some(title) = path
                .file_stem()
                .and_then(|value| value.to_str())
                .map(str::to_string)
            else {
                continue;
            };
            if !is_untitled_storage_title(&title) {
                continue;
            }
            if fs::read_to_string(path)
                .await
                .map_err(AppError::internal)?
                .is_empty()
            {
                return Ok(Some(title));
            }
        }
        Ok(None)
    }

    async fn allocate_untitled_title(&self, user_id: i64) -> AppResult<String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        for attempt in 0..1000u32 {
            let title = format!("{UNTITLED_TITLE_PREFIX}{timestamp}-{attempt}");
            if !fs::try_exists(self.path_for(user_id, &title))
                .await
                .map_err(AppError::internal)?
            {
                return Ok(title);
            }
        }
        Err(AppError::Internal(
            "Unable to allocate an untitled note filename.".to_string(),
        ))
    }

    fn path_for(&self, user_id: i64, title: &str) -> PathBuf {
        self.user_storage_path(user_id).join(format!("{title}.md"))
    }

    async fn note_from_file(&self, user_id: i64, title: &str, content: String) -> AppResult<Note> {
        let modified = fs::metadata(self.path_for(user_id, title))
            .await
            .map_err(AppError::internal)?
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH);
        Ok(Note {
            title: title.to_string(),
            content: Some(content),
            last_modified: modified
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
        })
    }
}

fn is_untitled_storage_title(title: &str) -> bool {
    title == DEFAULT_NOTE_TITLE || title.starts_with(UNTITLED_TITLE_PREFIX)
}

fn validate_title(title: &str) -> AppResult<String> {
    let value = title.trim();
    if value.is_empty()
        || value.chars().any(|character| {
            matches!(
                character,
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
            ) || character.is_control()
        })
    {
        return Err(AppError::BadRequest(
            "The specified note title contains invalid characters.".to_string(),
        ));
    }
    Ok(value.to_string())
}

async fn write_file(path: &PathBuf, content: &str) -> AppResult<()> {
    fs::write(path, content).await.map_err(AppError::internal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn notes_are_private_to_their_owner() {
        let root = std::env::temp_dir().join(format!("mionote-note-test-{}", std::process::id()));
        let store = NoteStore::new(root.clone()).await.unwrap();
        store
            .create(
                101,
                NoteCreate {
                    title: "私密笔记".to_string(),
                    content: Some("只属于账户 101".to_string()),
                },
            )
            .await
            .unwrap();

        assert!(store.get(101, "私密笔记").await.is_ok());
        assert!(matches!(
            store.get(202, "私密笔记").await,
            Err(AppError::NotFound(_))
        ));
        assert!(!store.user_storage_path(202).join("私密笔记.md").exists());
        let _ = tokio::fs::remove_dir_all(root).await;
    }

    #[tokio::test]
    async fn untitled_notes_get_unique_storage_titles_and_reuse_an_empty_note() {
        let root =
            std::env::temp_dir().join(format!("mionote-untitled-note-test-{}", std::process::id()));
        let store = NoteStore::new(root.clone()).await.unwrap();

        let content_note = store
            .create(
                101,
                NoteCreate {
                    title: DEFAULT_NOTE_TITLE.to_string(),
                    content: Some("已有正文".to_string()),
                },
            )
            .await
            .unwrap();
        let second_content_note = store
            .create(
                101,
                NoteCreate {
                    title: DEFAULT_NOTE_TITLE.to_string(),
                    content: Some("另一篇正文".to_string()),
                },
            )
            .await
            .unwrap();
        assert!(second_content_note.title.starts_with(UNTITLED_TITLE_PREFIX));
        let titled_empty_note = store
            .create(
                101,
                NoteCreate {
                    title: "待命笔记".to_string(),
                    content: Some(String::new()),
                },
            )
            .await
            .unwrap();

        let empty_untitled_note = store
            .update(
                101,
                &titled_empty_note.title,
                NoteUpdate {
                    new_title: Some(DEFAULT_NOTE_TITLE.to_string()),
                    new_content: None,
                },
            )
            .await
            .unwrap();
        assert!(empty_untitled_note.title.starts_with(UNTITLED_TITLE_PREFIX));
        assert_eq!(content_note.content.as_deref(), Some("已有正文"));

        let another_titled_empty_note = store
            .create(
                101,
                NoteCreate {
                    title: "另一篇空标题候选".to_string(),
                    content: Some(String::new()),
                },
            )
            .await
            .unwrap();
        assert!(matches!(
            store
                .update(
                    101,
                    &another_titled_empty_note.title,
                    NoteUpdate {
                        new_title: Some(DEFAULT_NOTE_TITLE.to_string()),
                        new_content: None,
                    },
                )
                .await,
            Err(AppError::Conflict(message)) if message == "Cannot create a second untitled empty note."
        ));

        let reopened_empty_note = store
            .create(
                101,
                NoteCreate {
                    title: DEFAULT_NOTE_TITLE.to_string(),
                    content: Some(String::new()),
                },
            )
            .await
            .unwrap();
        assert_eq!(reopened_empty_note.title, empty_untitled_note.title);

        let _ = tokio::fs::remove_dir_all(root).await;
    }
}
