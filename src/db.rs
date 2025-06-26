// src/db.rs

use sqlx::{PgPool, Error as SqlxError};
use crate::models::{User, ChatMessage};

/// Creates a new user in the `users` table with a specific role.
pub async fn create_user(pool: &PgPool, username: &str, password_hash: &str, role: &str) -> Result<User, SqlxError> {
    // Switched to the runtime-checked version of query_as to avoid compile-time DB connection issues.
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3)
         RETURNING id, username, password_hash, role, created_at"
    )
    .bind(username)
    .bind(password_hash)
    .bind(role)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Finds a user by their username.
pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, SqlxError> {
    // Switched to the runtime-checked version of query_as.
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, role, created_at FROM users WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Saves an encrypted chat message in the `messages` table.
pub async fn create_message(
    pool: &PgPool,
    user_id: i32,
    username: &str,
    encrypted_message: &[u8]
) -> Result<(), SqlxError> {
    // This function already uses a runtime-checked query.
    sqlx::query("INSERT INTO messages (user_id, username, message_text) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind(username)
        .bind(encrypted_message)
        .execute(pool)
        .await?;

    Ok(())
}

/// Retrieves the most recent (still encrypted) chat messages.
pub async fn get_recent_messages(pool: &PgPool, limit: i64) -> Result<Vec<ChatMessage>, SqlxError> {
    // Switched to the runtime-checked version of query_as.
    let messages = sqlx::query_as::<_, ChatMessage>(
        "SELECT id, user_id, username, message_text, created_at
         FROM messages
         ORDER BY created_at DESC
         LIMIT $1"
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(messages)
}
