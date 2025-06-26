use actix_web::{http::header, HttpRequest};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use sqlx::PgPool;
use std::env;
use crate::{db, errors::AppError, models::User};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|_| AppError::InternalServerError("Failed to verify password".to_string()))
}

pub async fn authenticate_user(req: HttpRequest, pool: &PgPool) -> Result<User, AppError> {
    let (username, password) = decode_basic_auth_header(req)?;

    let user = db::find_user_by_username(pool, &username)
        .await?
        .ok_or(AppError::Unauthorized("Invalid username or password".to_string()))?;
    
    if !verify_password(&password, &user.password_hash)? {
        return Err(AppError::Unauthorized("Invalid username or password".to_string()));
    }

    Ok(user)
}

pub async fn authenticate_admin(req: HttpRequest) -> Result<(), AppError> {
    let (username, password) = decode_basic_auth_header(req)?;

    let admin_user = env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set");
    let admin_pass = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");

    if username == admin_user && password == admin_pass {
        Ok(())
    } else {
        Err(AppError::Forbidden("You are not authorized to perform this action.".to_string()))
    }
}

fn decode_basic_auth_header(req: HttpRequest) -> Result<(String, String), AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AppError::Unauthorized("Missing Authorization header".to_string()))?
        .to_str()
        .map_err(|_| AppError::BadRequest("Invalid Authorization header format".to_string()))?;

    if !auth_header.starts_with("Basic ") {
        return Err(AppError::Unauthorized("Invalid authentication scheme".to_string()));
    }

    let base64_creds = &auth_header["Basic ".len()..];
    let decoded_creds = STANDARD.decode(base64_creds)
        .map_err(|_| AppError::BadRequest("Invalid Base64 in credentials".to_string()))?;
    let creds_str = String::from_utf8(decoded_creds)
        .map_err(|_| AppError::BadRequest("Invalid UTF-8 in credentials".to_string()))?;

    let mut parts = creds_str.splitn(2, ':');
    let username = parts.next().ok_or(AppError::BadRequest("Missing username".to_string()))?;
    let password = parts.next().ok_or(AppError::BadRequest("Missing password".to_string()))?;

    Ok((username.to_string(), password.to_string()))
}
