# `ErrorHandlers` Implementation

This version creates a handler for `actix_web::http::StatusCode::NOT_FOUND` using `actix_web::middleware::errhandlers::ErrorHandlers`.  The `not_found` function is adapted from the [actix_todo](https://github.com/actix/examples/tree/master/actix_todo) example.


## Issues

The `not_found` handler works as expected when using a hardcoded path to 404.html but we actually want to be passing in a PathBuf like `static_root.join("404.html")` in the old `NotFoundHandler`.  I haven't yet found a way to get this into the handler.
