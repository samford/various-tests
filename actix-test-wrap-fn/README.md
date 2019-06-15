# `wrap_fn` Implementation

This version creates a handler that only operates with the `actix_web::http::StatusCode::NOT_FOUND` status code, in a fashion that's somewhat similar to the old `NotFoundHandler`.


## Issues

The handler works as expected when using a hardcoded path to 404.html but we actually want to be passing in a PathBuf like `static_root.join("404.html")` in the old `NotFoundHandler`.  I haven't yet found a way to get this into the handler.
