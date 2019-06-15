extern crate actix_files;
extern crate actix_web;

use actix_files as fs;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn not_found(
    _req: HttpRequest,
    rendered_template: web::Data<PathBuf>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut fh = File::open(rendered_template.get_ref()).unwrap();
    let mut buf: Vec<u8> = vec![];
    let _ = fh.read_to_end(&mut buf);

    Ok(HttpResponse::build(http::StatusCode::NOT_FOUND)
        .header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("text/html"),
        )
        .body(buf)
        .into_body())
}

fn main() -> io::Result<()> {
    let address = "127.0.0.1:8080";

    // NOTE: output_dir is normally determined at runtime but it's hardcoded
    // here for simplicity
    let output_dir = "public";
    let static_root = Path::new(output_dir).to_path_buf();

    let sys = actix_rt::System::new("actix-test-wrap-fn");

    HttpServer::new(move || {
        App::new()
            .service(
                fs::Files::new("/", &static_root)
                    .show_files_listing()
                    .index_file("index.html"),
            )
            .data(static_root.join("404.html"))
            .default_service(web::resource("").route(web::to(not_found)))
    })
    .bind(address)?
    .start();

    println!("Starting http server {}\n", address);
    sys.run()
}
