use std::io::Write;
use std::path::Path;

use crate::config;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

use image::io::Reader as ImageReader;

pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // There should only be one part in the form
    let mut field = payload.try_next().await?.unwrap();

    // A multipart/form-data stream has to contain `content_disposition`
    let content_disposition = field
        .content_disposition()
        .ok_or_else(|| HttpResponse::BadRequest().finish())?;

    let name = content_disposition.get_name().unwrap();
    if name != "image" {
        return Ok(HttpResponse::BadRequest().body("Invalid form data"));
    }

    let file_storage_path = config::get_file_storage_path();

    let file_id = Uuid::new_v4().to_string();

    let filepath = format!("{}/{}.png", file_storage_path, file_id);

    assert!(!Path::exists(Path::new(&filepath)));

    let filepath_copy = filepath.clone();
    // File::create is blocking operation, use threadpool
    let mut f = web::block(|| std::fs::File::create(filepath_copy))
        .await
        .unwrap();

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // filesystem operations are blocking, we have to use threadpool
        f = web::block(move || f.write_all(&data).map(|_| f))
            .await
            .unwrap();
    }

    match ImageReader::open(&filepath)?.decode() {
        Ok(img) => println!("{:?}", img),
        Err(_err) => {
            // Delete the file if not an image
            std::fs::remove_file(&filepath).unwrap();
            return Ok(HttpResponse::BadRequest().body("Invalid image"));
        }
    }

    Ok(HttpResponse::Ok().body(file_id))
}
