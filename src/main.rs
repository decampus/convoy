use actix_cors::Cors;
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Result, http::header};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;
use hex;

mod auth;
mod db;
mod errors;
mod handlers;
mod models;
mod crypto;


pub struct AppState {
    db_pool: sqlx::PgPool,
}

/// Handler to serve the admin panel HTML file.
async fn admin_panel(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/admin.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("🚀 Server starting...");

    let key_hex = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set in .env file");
    if hex::decode(&key_hex).unwrap().len() != 32 {
        panic!("FATAL: ENCRYPTION_KEY must be 32 bytes long (64 hex characters).");
    }
    println!("Encryption key found and has correct length.");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool.");
    println!("Database connection pool created.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations.");
    println!("Database migrations ran successfully.");
    
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        // Ensure admin user exists in DB on startup
        let admin_user = env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set");
        let admin_pass = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");

        if db::find_user_by_username(&pool_clone, &admin_user).await.unwrap().is_none() {
            println!("Admin user not found in DB. Creating...");
            let hashed_password = auth::hash_password(&admin_pass).unwrap();
            db::create_user(&pool_clone, &admin_user, &hashed_password, "admin")
                .await
                .expect("Failed to create admin user in database");
            println!("Admin user '{}' created.", admin_user);
        } else {
             println!("Admin user '{}' already exists.", admin_user);
        }
    });

    println!("Starting HTTP server at http://127.0.0.1:8080");
    println!("Admin panel available at http://127.0.0.1:8080/admin");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState { db_pool: pool.clone() }))
            .service(
                web::scope("/api")
                    .route("/admin/login", web::post().to(handlers::admin_login))
                    .route("/admin/create_user", web::post().to(handlers::create_user_by_admin))
                    .route("/messages", web::post().to(handlers::post_message))
                    .route("/messages", web::get().to(handlers::get_messages)),
            )
            .route("/admin", web::get().to(admin_panel))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}