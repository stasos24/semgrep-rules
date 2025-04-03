use actix_web::{get, web, App, HttpServer};
use std::fs::File;

#[get("/file/{filename}")]
async fn get_file(filename: web::Path<String>) {
    let path = filename.into_inner();
    File::open(path).unwrap(); // Tainted 'path' flows into File::open
}
