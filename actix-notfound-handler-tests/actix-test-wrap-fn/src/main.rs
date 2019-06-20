extern crate actix_files;
extern crate actix_service;
extern crate actix_web;
extern crate futures;

use actix_files as fs;
use actix_service::Service;
use actix_web::{http, App, HttpResponse, HttpServer};
use futures::Future;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let address = "127.0.0.1:8080";

    // NOTE: output_dir is normally determined at runtime but it's hardcoded
    // here for simplicity
    let output_dir = "public";
    let static_root = Path::new(output_dir).to_path_buf();

    let sys = actix_rt::System::new("actix-test-wrap-fn");

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                srv.call(req).map(|mut res| {
                    if res.status() != http::StatusCode::NOT_FOUND {
                        return res;
                    }
                    res.headers_mut().insert(
                        http::header::CONTENT_TYPE,
                        http::header::HeaderValue::from_static("text/html"),
                    );
                    res.map_body(|_head, _body| {
                        // NOTE: This works when the path to 404.html is
                        // hardcoded but we need to somehow pass the
                        // static_root PathBuf to wrap_fn instead
                        let mut fh = File::open("public/404.html").unwrap();
                        let mut buf: Vec<u8> = vec![];
                        let _ = fh.read_to_end(&mut buf);
                        HttpResponse::build(http::StatusCode::NOT_FOUND)
                            .body(buf)
                            .take_body()
                    })
                })
            })
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
