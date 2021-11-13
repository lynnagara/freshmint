use std::env;
use std::iter::Iterator;

use actix_web::{http::{StatusCode}, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_multipart::{Field, Multipart};
use futures::{StreamExt, TryStreamExt};

async fn index() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../client/dist/index.html"))
}

async fn js_bundle() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("application/javascript; charset=utf-8")
        .body(include_str!("../../client/dist/bundle.js"))
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("gm")
}

async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // There should only be one part in the form
    let mut field = payload.try_next().await?.unwrap();

    // A multipart/form-data stream has to contain `content_disposition`
    let _content_disposition = field
        .content_disposition()
        .ok_or_else(|| HttpResponse::BadRequest().finish())?;


    let chunk = field.next().await.unwrap();
    println!("{:?}", chunk);

    Ok(HttpResponse::Ok().body("blah"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").expect("Missing HOST");
    let port = env::var("PORT").expect("Missing PORT");
    let url = format!("{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/bundle.js", web::get().to(js_bundle))
            .route("/hello", web::get().to(hello))
            .route("/upload", web::post().to(upload))
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
    async fn test_upload() {
        let mut app = test::init_service(App::new().route("/upload", web::post().to(upload))).await;
        let req = test::TestRequest::post()
            .set_payload("gm")
            .uri("/upload")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(test::read_body(resp).await, "gm");
    }
}
