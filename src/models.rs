use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, FromRow)]
pub struct ChatMessage {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub message_text: Vec<u8>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Serialize)]
pub struct ChatMessageResponse {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub message_text: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct NewMessagePayload {
    pub message_text: String,
}