use std::fs;
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    let html_content = fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| "<h1>Page unavaliable</h1>".to_string());

    actix_web::HttpResponse::Ok().content_type("text/html").body(html_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}