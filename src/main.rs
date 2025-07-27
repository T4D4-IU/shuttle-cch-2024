use actix_web::{get, http::header::LOCATION, web::{self, ServiceConfig}, HttpResponse, Responder};
use shuttle_actix_web::ShuttleActixWeb;
use std::net::{Ipv4Addr, Ipv6Addr};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Ipv4DestParams {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

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

#[get("/2/dest")]
async fn calcurate_dest(params: web::Query<Ipv4DestParams>) -> impl Responder {
    let from_octets = params.from.octets();
    let key_octets = params.to.octets();

    let mut dest_octets = [0u8; 4];
    for i in 0..4 {
        dest_octets[i] = from_octets[i].wrapping_add(key_octets[i]);
    }
    Ipv4Addr::from(dest_octets).to_string()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_bird);
        cfg.service(seek);
    };

    Ok(config.into())
}
