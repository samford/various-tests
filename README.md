# Actix `NOT_FOUND` Handler Tests

These are small, standalone tests which attempt to implement a handler for `actix_web::http::StatusCode::NOT_FOUND`, using either `actix_web::middleware::errhandlers::ErrorHandlers`, middleware, or `wrap_fn`.  Currently only the `ErrorHandlers` and `wrap_fn` implementations compile and run properly (albeit, with the caveat mentioned below).

These could probably all be improved but I've mostly been focused on producing something functional (before refactoring).


## Issues

* The `ErrorHandlers` and `wrap_fn` implementations both use a string literal to specify the path to 404.html, whereas we actually want to be providing this to the handler somehow (like the old `static_root.join("404.html")` provided to `NotFoundHandler`).

* The middleware implementation produces a lifetime error that I haven't been able to resolve, so that implementation is stuck there.
