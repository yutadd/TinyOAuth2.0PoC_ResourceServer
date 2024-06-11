use crate::mods::config::config::CONFIG;
use crate::mods::services::token::exchange_token;
use crate::mods::util::response::versatale::return_error_page;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode};
use std::convert::Infallible;
pub async fn hello(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = request.uri().path().to_string();
    let method = request.method().clone();
    if method.as_str() == "GET" {
        if path == CONFIG.endpoints.get_resource {
            return Ok(exchange_token(request).await);
        } 
    }
    println!("return 404 because path that the user requested is invalid.");
    Ok(return_error_page(StatusCode::NOT_FOUND, "path not found."))
}
