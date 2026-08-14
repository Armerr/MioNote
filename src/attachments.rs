use std::path::PathBuf;

use axum::{
    body::Body,
    extract::Multipart,
    http::{header, HeaderMap, HeaderValue},
    response::{IntoResponse, Response},
};
use chrono::Utc;
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    error::{AppError, AppResult},
    models::AttachmentResponse,
};

const MAX_ATTACHMENT_BYTES: usize = 25 * 1024 * 1024;

pub struct AttachmentStore {
    base_path: PathBuf,
}

impl AttachmentStore {
    pub async fn new(base_path: PathBuf) -> AppResult<Self> {
        Ok(Self { base_path })
    }

    pub async fn create(
        &self,
        user_id: i64,
        mut multipart: Multipart,
    ) -> AppResult<AttachmentResponse> {
        while let Some(field) = multipart.next_field().await.map_err(AppError::internal)? {
            if field.name() != Some("file") {
                continue;
            }
            let original = field
                .file_name()
                .ok_or_else(|| {
                    AppError::BadRequest("An attachment filename is required.".to_string())
                })?
                .to_string();
            validate_filename(&original)?;
            let bytes = field.bytes().await.map_err(AppError::internal)?;
            if bytes.len() > MAX_ATTACHMENT_BYTES {
                return Err(AppError::PayloadTooLarge(
                    "The attachment is too large.".to_string(),
                ));
            }
            let filename = self.save_unique(user_id, &original, &bytes).await?;
            return Ok(AttachmentResponse {
                url: format!("attachments/{}", url_encode(&filename)),
                filename,
            });
        }
        Err(AppError::BadRequest(
            "The multipart request must contain a file field.".to_string(),
        ))
    }

    async fn save_unique(&self, user_id: i64, original: &str, bytes: &[u8]) -> AppResult<String> {
        let storage_path = self.storage_path(user_id);
        fs::create_dir_all(&storage_path)
            .await
            .map_err(AppError::internal)?;
        let (stem, extension) = split_extension(original);
        for attempt in 0..100u16 {
            let filename = if attempt == 0 {
                original.to_string()
            } else {
                format!(
                    "{}_{}-{}{}",
                    stem,
                    Utc::now().format("%Y-%m-%dT%H-%M-%SZ"),
                    attempt,
                    extension
                )
            };
            let path = storage_path.join(&filename);
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .await
            {
                Ok(mut file) => {
                    file.write_all(bytes).await.map_err(AppError::internal)?;
                    return Ok(filename);
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(AppError::internal(error)),
            }
        }
        Err(AppError::Conflict(
            "Cannot create attachment. Too many duplicate filenames.".to_string(),
        ))
    }

    pub async fn get(&self, user_id: i64, filename: &str) -> AppResult<Response> {
        validate_filename(filename)?;
        let path = self.storage_path(user_id).join(filename);
        let bytes = fs::read(&path).await.map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                AppError::NotFound("The specified attachment cannot be found.".to_string())
            } else {
                AppError::internal(error)
            }
        })?;
        let content_type: HeaderValue = mime_guess::from_path(&path)
            .first_or_octet_stream()
            .essence_str()
            .parse()
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, content_type);
        headers.insert(
            header::CONTENT_LENGTH,
            HeaderValue::from_str(&bytes.len().to_string()).unwrap(),
        );
        Ok((headers, Body::from(bytes)).into_response())
    }

    fn storage_path(&self, user_id: i64) -> PathBuf {
        self.base_path
            .join("users")
            .join(user_id.to_string())
            .join("attachments")
    }
}

fn validate_filename(filename: &str) -> AppResult<()> {
    if filename.is_empty()
        || filename.contains("..")
        || filename.chars().any(|character| {
            matches!(
                character,
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
            ) || character.is_control()
        })
    {
        return Err(AppError::BadRequest(
            "The specified filename contains invalid characters.".to_string(),
        ));
    }
    Ok(())
}

fn split_extension(filename: &str) -> (String, String) {
    match filename.rsplit_once('.') {
        Some((stem, extension)) => (stem.to_string(), format!(".{extension}")),
        None => (filename.to_string(), String::new()),
    }
}

fn url_encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}
