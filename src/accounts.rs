use std::{
    path::Path,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use rusqlite::{params, Connection, OptionalExtension};

use crate::{
    error::{AppError, AppResult},
    models::Register,
};

#[derive(Clone, Debug)]
pub struct Account {
    pub id: i64,
    pub username: String,
    password_hash: String,
}

pub struct AccountStore {
    connection: Mutex<Connection>,
}

impl AccountStore {
    pub fn new(path: &Path) -> AppResult<Self> {
        let connection = Connection::open(path).map_err(AppError::internal)?;
        connection
            .execute_batch(
                "
                PRAGMA journal_mode = WAL;
                PRAGMA foreign_keys = ON;
                CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username TEXT NOT NULL COLLATE NOCASE UNIQUE,
                    password_hash TEXT NOT NULL,
                    created_at INTEGER NOT NULL
                );
                ",
            )
            .map_err(AppError::internal)?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn register(&self, data: Register) -> AppResult<Account> {
        let username = validate_username(&data.username)?;
        validate_password(&data.password)?;
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(data.password.as_bytes(), &salt)
            .map_err(AppError::internal)?
            .to_string();
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let connection = self
            .connection
            .lock()
            .map_err(|_| AppError::internal("账户数据库锁不可用"))?;
        match connection.execute(
            "INSERT INTO users (username, password_hash, created_at) VALUES (?1, ?2, ?3)",
            params![username, password_hash, created_at],
        ) {
            Ok(_) => Ok(Account {
                id: connection.last_insert_rowid(),
                username,
                password_hash,
            }),
            Err(error) if error.to_string().contains("UNIQUE constraint failed") => {
                Err(AppError::Conflict("该用户名已被使用。".to_string()))
            }
            Err(error) => Err(AppError::internal(error)),
        }
    }

    pub fn verify(&self, username: &str, password: &str) -> AppResult<Account> {
        let account = self
            .by_username(username)?
            .ok_or_else(|| AppError::Unauthorized("用户名或密码错误。".to_string()))?;
        let parsed = PasswordHash::new(&account.password_hash)
            .map_err(|_| AppError::Unauthorized("用户名或密码错误。".to_string()))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AppError::Unauthorized("用户名或密码错误。".to_string()))?;
        Ok(account)
    }

    pub fn by_id(&self, id: i64) -> AppResult<Option<Account>> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| AppError::internal("账户数据库锁不可用"))?;
        connection
            .query_row(
                "SELECT id, username, password_hash FROM users WHERE id = ?1",
                params![id],
                row_to_account,
            )
            .optional()
            .map_err(AppError::internal)
    }

    fn by_username(&self, username: &str) -> AppResult<Option<Account>> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| AppError::internal("账户数据库锁不可用"))?;
        connection
            .query_row(
                "SELECT id, username, password_hash FROM users WHERE username = ?1",
                params![username.trim()],
                row_to_account,
            )
            .optional()
            .map_err(AppError::internal)
    }
}

fn row_to_account(row: &rusqlite::Row<'_>) -> rusqlite::Result<Account> {
    Ok(Account {
        id: row.get(0)?,
        username: row.get(1)?,
        password_hash: row.get(2)?,
    })
}

fn validate_username(value: &str) -> AppResult<String> {
    let username = value.trim();
    let length = username.chars().count();
    if !(3..=32).contains(&length)
        || username
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(AppError::BadRequest(
            "用户名长度应为 3 至 32 个字符，且不能包含空白字符。".to_string(),
        ));
    }
    Ok(username.to_string())
}

fn validate_password(value: &str) -> AppResult<()> {
    if value.chars().count() < 8 {
        return Err(AppError::BadRequest("密码至少需要 8 个字符。".to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_and_verifies_unicode_username() {
        let database_path = std::env::temp_dir().join(format!(
            "mionote-account-test-{}-{}.db",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let accounts = AccountStore::new(&database_path).unwrap();
        let account = accounts
            .register(Register {
                username: "小米用户".to_string(),
                password: "a-secure-password".to_string(),
            })
            .unwrap();
        assert_eq!(account.username, "小米用户");
        assert_eq!(
            accounts.verify("小米用户", "a-secure-password").unwrap().id,
            account.id
        );
        let _ = std::fs::remove_file(database_path);
    }
}
