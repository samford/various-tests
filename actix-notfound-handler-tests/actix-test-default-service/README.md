# `default_service` Implementation

This version creates a `default_service` that returns a `NOT_FOUND` response using a static 404.html file (where the path can be determined at runtime).  With this setup, we're able to provide `static_root.join("404.html")` using `actix_web::web::Data` before `default_service`.


## Issues

This implementation doesn't distinguish between errors and simply returns a `NOT_FOUND` status for any requests that aren't successful.  Ideally we would properly handle different errors (e.g., 4xx, 5xx errors).
