# Actix `NOT_FOUND` Handler Tests

These are small, standalone tests which attempt to implement a handler for `actix_web::http::StatusCode::NOT_FOUND`, using `default_service`, `actix_web::middleware::errhandlers::ErrorHandlers`, middleware, or `wrap_fn`.  Currently only the `default_service`, `ErrorHandlers`, and `wrap_fn` implementations compile and run properly (albeit, with caveats mentioned below).

These could probably all be improved but I've mostly been focused on producing something functional (before refactoring).


## Issues

* The `default_service` implementation doesn't distinguish between errors and simply returns a `NOT_FOUND` status for any requests that aren't successful.  Ideally we would properly handle different errors (e.g., 4xx, 5xx errors) but maybe this shortcoming is fine in this context (since this is likely just a development server and wouldn't be used in production).

* `wrap_fn` implementation uses a string literal to specify the path to 404.html, whereas we actually want to be providing this to the handler somehow (like the old `static_root.join("404.html")` provided to `NotFoundHandler`).

* The middleware implementation produces a lifetime error that I haven't been able to resolve, so that implementation is stuck there.
