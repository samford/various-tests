extern crate actix_files;
extern crate actix_web;

use actix_files as fs;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, http, App, HttpResponse, HttpServer};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

struct ErrorFilePaths {
    not_found: PathBuf,
}

fn not_found<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, actix_web::Error> {
    let error_files: &ErrorFilePaths = res.request().app_data().unwrap();

    let mut fh = File::open(&error_files.not_found)?;
    let mut buf: Vec<u8> = vec![];
    let _ = fh.read_to_end(&mut buf)?;

    let new_resp = HttpResponse::build(http::StatusCode::NOT_FOUND)
        .header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("text/html"),
        )
        .body(buf);

    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

fn main() -> io::Result<()> {
    let address = "127.0.0.1:8080";

    // NOTE: output_dir is normally determined at runtime but it's hardcoded
    // here for simplicity
    let output_dir = "public";
    let static_root = Path::new(output_dir).to_path_buf();

    let sys = actix_rt::System::new("actix-test-errorhandlers");

    HttpServer::new(move || {
        let error_handlers = ErrorHandlers::new()
            .handler(http::StatusCode::NOT_FOUND, not_found);

        App::new()
            .data(ErrorFilePaths {
                not_found: static_root.join("404.html"),
            })
            .wrap(error_handlers)
            .service(
                fs::Files::new("/", &static_root)
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind(address)?
    .start();

    println!("Starting http server {}\n", address);
    sys.run()
}
