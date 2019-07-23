#[macro_use]
extern crate lazy_static;

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer, Result};
use serde_json::json;
use std::fs;

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

fn music(req: HttpRequest) -> Result<NamedFile> {
    let file = format!("./assets/music/{}", req.match_info().get("file").unwrap());
    Ok(NamedFile::open(file)?)
}

fn list_music() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json!(&*MUSICS.lock().unwrap()).to_string())
}

fn refresh_music() -> HttpResponse {
    update_music();

    // just ok
    // ;)
    HttpResponse::Ok().content_type("text/plain").body("ok")
}

fn update_music() {
    // empty/truncate a Vector, then append all listed music
    // another way is to not truncate the vector, but just append a new file
    // but if there any deleted/updated file, so vector value will not updated
    MUSICS.lock().unwrap().truncate(0);

    let paths = fs::read_dir("./assets/music").unwrap();

    for path in paths {
        let file_name = path.unwrap().file_name().into_string().unwrap();
        if file_name.contains(".mp3") {
            MUSICS.lock().unwrap().push(file_name);
        }
    }
}

fn main() {
    update_music();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/music", web::get().to(list_music))
            .route("/refresh", web::get().to(refresh_music))
            .route("/assets/music/{file}", web::get().to(music))
            .route("/assets/{file}", web::get().to(assets))
    })
    .bind("127.0.0.1:3003")
    .unwrap()
    .run()
    .unwrap();
}
