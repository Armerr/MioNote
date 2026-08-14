use std::{env, net::IpAddr, path::PathBuf};

use crate::error::{AppError, AppResult};

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub path_prefix: String,
    pub storage_path: PathBuf,
    pub secret_key: String,
    pub session_expiry_days: i64,
    pub registration_open: bool,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        let path_prefix = env_string("MIONOTE_PATH_PREFIX", "")?;
        if !path_prefix.is_empty() && (!path_prefix.starts_with('/') || path_prefix.ends_with('/'))
        {
            return Err(AppError::Internal(
                "MIONOTE_PATH_PREFIX must start with '/' and not end with '/'.".to_string(),
            ));
        }

        let storage_path = PathBuf::from(required_env("MIONOTE_PATH")?);

        Ok(Self {
            host: env_string("MIONOTE_HOST", "0.0.0.0")?,
            port: env_string("MIONOTE_PORT", "4233")?.parse().map_err(|_| {
                AppError::Internal("MIONOTE_PORT must be a valid port number.".to_string())
            })?,
            path_prefix,
            storage_path,
            secret_key: required_env("MIONOTE_SECRET_KEY")?,
            session_expiry_days: env_string("MIONOTE_SESSION_EXPIRY_DAYS", "30")?
                .parse()
                .map_err(|_| {
                    AppError::Internal(
                        "MIONOTE_SESSION_EXPIRY_DAYS must be an integer.".to_string(),
                    )
                })?,
            registration_open: env_bool("MIONOTE_REGISTRATION_OPEN", true)?,
        })
    }

    pub fn bind_address(&self) -> AppResult<std::net::SocketAddr> {
        let ip: IpAddr = self.host.parse().map_err(|_| {
            AppError::Internal("MIONOTE_HOST must be a valid IP address.".to_string())
        })?;
        Ok((ip, self.port).into())
    }
}

fn env_string(key: &str, default: &str) -> AppResult<String> {
    Ok(env::var(key).unwrap_or_else(|_| default.to_string()))
}

fn required_env(key: &str) -> AppResult<String> {
    env::var(key)
        .map_err(|_| AppError::Internal(format!("Environment variable {key} must be set.")))
}

fn env_bool(key: &str, default: bool) -> AppResult<bool> {
    match env::var(key) {
        Ok(value) => match value.to_ascii_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(AppError::Internal(format!(
                "Invalid value '{value}' for {key}."
            ))),
        },
        Err(_) => Ok(default),
    }
}
