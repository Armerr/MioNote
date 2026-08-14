use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::http::{header, HeaderMap};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    accounts::{Account, AccountStore},
    config::Config,
    error::{AppError, AppResult},
    models::{Login, Register, Token},
};

#[derive(Clone)]
pub struct AuthService {
    config: Arc<Config>,
    accounts: Arc<AccountStore>,
}

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl AuthService {
    pub fn new(config: Arc<Config>, accounts: Arc<AccountStore>) -> Self {
        Self { config, accounts }
    }

    pub fn login(&self, data: Login) -> AppResult<Token> {
        let account = self.accounts.verify(&data.username, &data.password)?;
        self.issue_token(&account)
    }

    pub fn register(&self, data: Register) -> AppResult<Token> {
        if !self.config.registration_open {
            return Err(AppError::Unauthorized("当前未开放注册。".to_string()));
        }
        let account = self.accounts.register(data)?;
        self.issue_token(&account)
    }

    pub fn require(&self, headers: &HeaderMap) -> AppResult<AuthenticatedUser> {
        let token = bearer_token(headers)
            .or_else(|| cookie_token(headers))
            .ok_or_else(|| AppError::Unauthorized("登录状态已失效，请重新登录。".to_string()))?;
        let claims = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.config.secret_key.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("登录状态已失效，请重新登录。".to_string()))?
        .claims;
        let id = claims
            .sub
            .parse::<i64>()
            .map_err(|_| AppError::Unauthorized("登录状态已失效，请重新登录。".to_string()))?;
        let account = self
            .accounts
            .by_id(id)?
            .ok_or_else(|| AppError::Unauthorized("登录状态已失效，请重新登录。".to_string()))?;
        Ok(AuthenticatedUser {
            id: account.id,
            username: account.username,
        })
    }

    fn issue_token(&self, account: &Account) -> AppResult<Token> {
        let expiry = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize
            + (self.config.session_expiry_days.max(1) as usize * 86_400);
        let claims = Claims {
            sub: account.id.to_string(),
            exp: expiry,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.secret_key.as_bytes()),
        )
        .map_err(AppError::internal)?;
        Ok(Token {
            access_token: token,
            token_type: "bearer",
        })
    }
}

fn bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(str::to_string)
}

fn cookie_token(headers: &HeaderMap) -> Option<String> {
    let value = headers.get(header::COOKIE)?.to_str().ok()?;
    value
        .split(';')
        .map(str::trim)
        .find_map(|item| item.strip_prefix("token=").map(str::to_string))
}
