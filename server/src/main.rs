use std::env;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("gm")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").expect("Missing HOST");
    let port = env::var("PORT").expect("Missing PORT");
    let url = format!("{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(url)?
    .run()
    .await
}