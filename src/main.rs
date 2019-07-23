#[macro_use]
extern crate lazy_static;

use std::fs;
use serde_json::json;
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use actix_files::NamedFile;
use actix_web::{App, web, HttpServer, Result};
use actix_web::http::{StatusCode};

use std::sync::Mutex;

lazy_static! {
    static ref MUSICS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

fn index(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

fn assets(req: HttpRequest) -> Result<NamedFile> {
    let file = format!("./assets/{}", req.match_info().get("file").unwrap());
    Ok(NamedFile::open(file)?)
}

fn list_music() -> HttpResponse {

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(&*MUSICS.lock().unwrap()).to_string())
}

fn refresh_music() -> HttpResponse {

    update_music();

    HttpResponse::Ok()
        .content_type("text/plain")
        .body("ok")
}

fn update_music() {
    let paths = fs::read_dir("./assets/music").unwrap();

    for path in paths {
        MUSICS.lock().unwrap().push(path.unwrap().file_name().into_string().unwrap());
    }
}

fn main() {

    update_music();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/music", web::get().to(list_music))
            .route("/refresh", web::get().to(refresh_music))
            .route("/assets/{file}", web::get().to(assets))
    })
    .bind("127.0.0.1:3003")
    .unwrap()
    .run()
    .unwrap();
}