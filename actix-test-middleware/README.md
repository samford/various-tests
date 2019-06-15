# Middleware Implementation

This version attempts to handle `actix_web::http::StatusCode::NOT_FOUND` errors using middleware, somewhat similarly to the old version (but requiring more boilerplate code).  The middleware code largely comes from the Actix [middleware](https://github.com/actix/examples/tree/master/middleware) example.

This project doesn't build at the moment due to an error (see below), so this implementation is currently stuck here.


## Issues

We're able to provide a PathBuf to 404.html to `NotFoundHandler` but the `call` function in `NotFoundMiddleware` runs into the following lifetime error:

```
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
   --> src/main.rs:73:50
    |
73  |           Box::new(self.service.call(req).and_then(|res| {
    |  __________________________________________________^
74  | |             if res.status() != http::StatusCode::NOT_FOUND {
75  | |                 return Ok(res);
76  | |             }
...   |
107 | |             ))
108 | |         }))
    | |_________^
    |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 72:5...
   --> src/main.rs:72:5
    |
72  | /     fn call(&mut self, req: dev::ServiceRequest) -> Self::Future {
73  | |         Box::new(self.service.call(req).and_then(|res| {
74  | |             if res.status() != http::StatusCode::NOT_FOUND {
75  | |                 return Ok(res);
...   |
108 | |         }))
109 | |     }
    | |_____^
    = note: ...so that the types are compatible:
            expected &&mut NotFoundMiddleware<S>
               found &&mut NotFoundMiddleware<S>
    = note: but, the lifetime must be valid for the static lifetime...
    = note: ...so that the expression is assignable:
            expected std::boxed::Box<(dyn futures::Future<Item = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error> + 'static)>
               found std::boxed::Box<dyn futures::Future<Item = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error>>
```
