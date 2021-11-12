use std::env;

use actix_web::{web,  App, HttpResponse, HttpServer, Responder};


async fn hello() -> impl Responder {
    HttpResponse::Ok().body("gm")
}

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
        .route("/", web::get().to(hello))
        .route("/echo", web::post().to(echo))
    })
    .bind(url)?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};


    #[actix_rt::test]
    async fn test_hello() {
        let mut app = test::init_service(App::new().route("/", web::get().to(hello))).await;
        let req = test::TestRequest::get().to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(test::read_body(resp).await, "gm");
    }

    #[actix_rt::test]
    async fn test_echo() {
        let mut app = test::init_service(App::new().route("/echo", web::post().to(echo))).await;
        let req = test::TestRequest::post().set_payload("gm").uri("/echo").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(test::read_body(resp).await, "gm");
    }
}