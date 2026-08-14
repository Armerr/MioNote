mod accounts;
mod attachments;
mod auth;
mod config;
mod error;
mod models;
mod notes;
mod search;

use std::sync::Arc;

use axum::{
    extract::{DefaultBodyLimit, Multipart, Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};
use tracing::info;

use crate::{
    accounts::AccountStore,
    attachments::AttachmentStore,
    auth::AuthService,
    config::Config,
    error::{AppError, AppResult},
    models::{
        AttachmentResponse, ConfigResponse, CurrentUser, Login, Note, NoteCreate, NoteUpdate,
        Register, SearchResult, Token,
    },
    notes::NoteStore,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub auth: Arc<AuthService>,
    pub notes: Arc<NoteStore>,
    pub attachments: Arc<AttachmentStore>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub term: String,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub limit: Option<usize>,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    let config = Arc::new(Config::from_env()?);
    tokio::fs::create_dir_all(&config.storage_path)
        .await
        .map_err(AppError::internal)?;
    let notes = Arc::new(NoteStore::new(config.storage_path.clone()).await?);
    let attachments = Arc::new(AttachmentStore::new(config.storage_path.clone()).await?);
    let accounts = Arc::new(AccountStore::new(&config.storage_path.join("mionote.db"))?);
    let auth = Arc::new(AuthService::new(config.clone(), accounts));
    let state = AppState {
        config: config.clone(),
        auth,
        notes,
        attachments,
    };
    let app = build_router(state);
    let address = config.bind_address()?;
    let listener = TcpListener::bind(address)
        .await
        .map_err(AppError::internal)?;
    info!(%address, "MioNote Rust backend listening");
    axum::serve(listener, app).await.map_err(AppError::internal)
}

fn build_router(state: AppState) -> Router {
    let api = Router::new()
        .route("/api/config", get(get_config))
        .route("/api/auth-check", get(auth_check))
        .route("/api/token", post(token))
        .route("/api/register", post(register))
        .route("/api/users/me", get(current_user))
        .route("/api/notes", post(create_note))
        .route(
            "/api/notes/{title}",
            get(get_note).patch(update_note).delete(delete_note),
        )
        .route("/api/search", get(search_notes))
        .route("/api/tags", get(get_tags))
        .route("/api/attachments", post(upload_attachment))
        .route("/api/attachments/{filename}", get(get_attachment))
        .route("/attachments/{filename}", get(get_attachment))
        .route("/docs", get(api_docs))
        .route("/openapi.json", get(openapi))
        .route("/", get(index))
        .route("/login", get(index))
        .route("/search", get(index))
        .route("/new", get(index))
        .route("/note/{title}", get(index))
        .fallback(static_file)
        .with_state(state.clone())
        .layer(DefaultBodyLimit::max(25 * 1024 * 1024))
        .layer(RequestBodyLimitLayer::new(25 * 1024 * 1024))
        .layer(TraceLayer::new_for_http());

    if state.config.path_prefix.is_empty() {
        api
    } else {
        Router::new().nest(&state.config.path_prefix, api)
    }
}

async fn index(State(state): State<AppState>) -> AppResult<Response> {
    let html = state.notes.index_html(&state.config.path_prefix).await?;
    let mut response = Html(html).into_response();
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache"),
    );
    Ok(response)
}

// Vite emits hashed file names like `index-CNBHbjGy.js`; only those can be
// cached immutably. Everything else must be revalidated so browsers never
// serve a stale index.html (and with it, stale bundle references).
fn is_hashed_asset(path: &str) -> bool {
    let Some(file_name) = path.rsplit('/').next() else {
        return false;
    };
    let Some((stem, _extension)) = file_name.rsplit_once('.') else {
        return false;
    };
    let Some((_, hash)) = stem.rsplit_once('-') else {
        return false;
    };
    hash.len() == 8
        && hash
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
}

async fn static_file(State(state): State<AppState>, uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    if path.is_empty() || path.contains("..") {
        return StatusCode::NOT_FOUND.into_response();
    }
    let file_path = std::path::Path::new("client/dist").join(path);
    match tokio::fs::read(&file_path).await {
        Ok(bytes) => {
            let content_type = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .essence_str()
                .parse::<HeaderValue>()
                .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, content_type);
            if is_hashed_asset(&path) {
                headers.insert(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=31536000, immutable"),
                );
            } else {
                headers.insert(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("no-cache"),
                );
            }
            (headers, bytes).into_response()
        }
        Err(_) => {
            let _ = state;
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

async fn api_docs() -> Html<&'static str> {
    Html(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>MioNote API</title></head><body><h1>MioNote API</h1><p>Open <a href=\"openapi.json\">openapi.json</a> for the API document.</p></body></html>",
    )
}

async fn openapi() -> Json<serde_json::Value> {
    Json(json!({
        "openapi": "3.1.0",
        "info": {"title": "MioNote API", "version": env!("CARGO_PKG_VERSION")},
        "paths": {
            "/api/config": {},
            "/api/auth-check": {},
            "/api/token": {},
            "/api/register": {},
            "/api/users/me": {},
            "/api/notes": {},
            "/api/notes/{title}": {},
            "/api/search": {},
            "/api/tags": {},
            "/api/attachments": {},
            "/api/attachments/{filename}": {}
        }
    }))
}

async fn get_config(State(state): State<AppState>) -> Json<ConfigResponse> {
    Json(ConfigResponse {
        auth_type: "password",
        registration_open: state.config.registration_open,
    })
}

async fn auth_check(State(state): State<AppState>, headers: HeaderMap) -> AppResult<&'static str> {
    state.auth.require(&headers)?;
    Ok("OK")
}

async fn token(State(state): State<AppState>, Json(login): Json<Login>) -> AppResult<Json<Token>> {
    Ok(Json(state.auth.login(login)?))
}

async fn register(
    State(state): State<AppState>,
    Json(registration): Json<Register>,
) -> AppResult<(StatusCode, Json<Token>)> {
    Ok((
        StatusCode::CREATED,
        Json(state.auth.register(registration)?),
    ))
}

async fn current_user(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<CurrentUser>> {
    let user = state.auth.require(&headers)?;
    Ok(Json(CurrentUser {
        id: user.id,
        username: user.username,
    }))
}

async fn get_note(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(title): Path<String>,
) -> AppResult<Json<Note>> {
    let user = state.auth.require(&headers)?;
    Ok(Json(state.notes.get(user.id, &title).await?))
}

async fn create_note(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(note): Json<NoteCreate>,
) -> AppResult<Json<Note>> {
    let user = state.auth.require(&headers)?;
    Ok(Json(state.notes.create(user.id, note).await?))
}

async fn update_note(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(title): Path<String>,
    Json(data): Json<NoteUpdate>,
) -> AppResult<Json<Note>> {
    let user = state.auth.require(&headers)?;
    Ok(Json(state.notes.update(user.id, &title, data).await?))
}

async fn delete_note(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(title): Path<String>,
) -> AppResult<StatusCode> {
    let user = state.auth.require(&headers)?;
    state.notes.delete(user.id, &title).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn search_notes(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SearchQuery>,
) -> AppResult<Json<Vec<SearchResult>>> {
    let user = state.auth.require(&headers)?;
    Ok(Json(state.notes.search(user.id, query).await?))
}

async fn get_tags(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<Vec<String>>> {
    let user = state.auth.require(&headers)?;
    Ok(Json(state.notes.tags(user.id).await?))
}

async fn upload_attachment(
    State(state): State<AppState>,
    headers: HeaderMap,
    multipart: Multipart,
) -> AppResult<Json<AttachmentResponse>> {
    let user = state.auth.require(&headers)?;
    Ok(Json(state.attachments.create(user.id, multipart).await?))
}

async fn get_attachment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(filename): Path<String>,
) -> AppResult<Response> {
    let user = state.auth.require(&headers)?;
    state.attachments.get(user.id, &filename).await
}
