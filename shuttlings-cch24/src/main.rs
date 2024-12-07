use actix_web::{get, http::header::LOCATION, web::ServiceConfig, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

#[get("/{id}/seek")]
async fn seek() -> HttpResponse {
    HttpResponse::Found()
        .insert_header((LOCATION, "https://www.youtube.com/watch?v=9Gc4QTqslN4"))
        .finish()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_bird);
        cfg.service(seek);
    };

    Ok(config.into())
}
