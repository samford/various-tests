extern crate actix_files;
extern crate actix_service;
extern crate actix_web;
extern crate futures;

use actix_files as fs;
use actix_service::{Service, Transform};
use actix_web::{dev, http, App, HttpResponse, HttpServer};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

struct NotFoundHandler {
    rendered_template: PathBuf,
}

impl<S, B> Transform<S> for NotFoundHandler
where
    S: Service<
        Request = dev::ServiceRequest,
        Response = dev::ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = dev::ServiceRequest;
    type Response = dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = NotFoundMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(NotFoundMiddleware {
            service,
            rendered_template: self.rendered_template.clone(),
        })
    }
}

struct NotFoundMiddleware<S> {
    service: S,
    rendered_template: PathBuf,
}

impl<S, B> Service for NotFoundMiddleware<S>
where
    S: Service<
        Request = dev::ServiceRequest,
        Response = dev::ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = dev::ServiceRequest;
    type Response = dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    // NOTE: This function runs into a lifetime error ("error[E0495]: cannot
    // infer an appropriate lifetime due to conflicting requirements"). This
    // happens regardless of whether I use the commented out code below or the
    // live code.  I haven't yet figured out how to resolve this error.
    fn call(&mut self, req: dev::ServiceRequest) -> Self::Future {
        Box::new(self.service.call(req).and_then(|res| {
            if res.status() != http::StatusCode::NOT_FOUND {
                return Ok(res);
            }

            // res.headers_mut().insert(
            //     http::header::CONTENT_TYPE,
            //     http::header::HeaderValue::from_static("text/html"),
            // );

            // res.map_body(|_head, _body| {
            //     let mut fh = File::open(&self.rendered_template).unwrap();
            //     let mut buf: Vec<u8> = vec![];
            //     let _ = fh.read_to_end(&mut buf);

            //     HttpResponse::build(http::StatusCode::NOT_FOUND)
            //         .body(buf)
            //         .take_body()
            // });

            // Ok(res)

            let mut fh = File::open(&self.rendered_template).unwrap();
            let mut buf: Vec<u8> = vec![];
            let _ = fh.read_to_end(&mut buf);

            Ok(req.into_response(
                HttpResponse::build(http::StatusCode::NOT_FOUND)
                    .header(
                        http::header::CONTENT_TYPE,
                        http::header::HeaderValue::from_static("text/html"),
                    )
                    .body(buf)
                    .into_body(),
            ))
        }))
    }
}

fn main() -> io::Result<()> {
    let address = "127.0.0.1:8080";

    // NOTE: output_dir is normally determined at runtime but it's hardcoded
    // here for simplicity
    let output_dir = "public";
    let static_root = Path::new(output_dir).to_path_buf();

    let sys = actix_rt::System::new("actix-test-middleware");

    HttpServer::new(move || {
        App::new()
            .wrap(NotFoundHandler {
                rendered_template: static_root.join("404.html"),
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
