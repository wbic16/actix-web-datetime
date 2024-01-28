// src/main.rs

// dependencies
use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;

// index route endpont handler; serves today's date and time as HTML
#[get("/")]
async fn today() -> impl Responder {
    let today: DateTime<Local> = Local::now();
    let resp_html = format!("<p>{}</p>", today);
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(resp_html)
}

// main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(today))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
