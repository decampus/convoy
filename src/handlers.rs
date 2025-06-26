// src/handlers.rs

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::{
    models::{CreateUserPayload, NewMessagePayload, ChatMessageResponse},
    AppState, db, auth, crypto,
    errors::AppError
};

pub async fn admin_login(req: HttpRequest) -> Result<HttpResponse, AppError> {
    auth::authenticate_admin(req).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Admin authenticated successfully."
    })))
}

pub async fn create_user_by_admin(req: HttpRequest, state: web::Data<AppState>, payload: web::Json<CreateUserPayload>) -> Result<HttpResponse, AppError> {
    auth::authenticate_admin(req).await?;

    let username = payload.username.clone();
    let password = payload.password.clone();
    
    let hashed_password = auth::hash_password(password.as_str())?;
    
    let new_user = db::create_user(&state.db_pool, &username, &hashed_password, "user").await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "message": "User created successfully by admin",
        "user": { "id": new_user.id, "username": new_user.username, "role": new_user.role }
    })))
}

pub async fn post_message(req: HttpRequest, state: web::Data<AppState>, payload: web::Json<NewMessagePayload>) -> Result<HttpResponse, AppError> {
    let user = auth::authenticate_user(req, &state.db_pool).await?;

    let encrypted_message = crypto::encrypt(&payload.message_text)?;
    
    db::create_message(&state.db_pool, user.id, &user.username, &encrypted_message).await?;
    
    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "message": "Message posted successfully.",
    })))
}

pub async fn get_messages(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let encrypted_messages = db::get_recent_messages(&state.db_pool, 100).await?;

    let mut decrypted_messages = Vec::new();
    for msg in encrypted_messages {
        let decrypted_text = crypto::decrypt(&msg.message_text)?;
        decrypted_messages.push(ChatMessageResponse {
            id: msg.id,
            user_id: msg.user_id,
            username: msg.username,
            message_text: decrypted_text,
            created_at: msg.created_at,
        });
    }

    Ok(web::Json(decrypted_messages))
}