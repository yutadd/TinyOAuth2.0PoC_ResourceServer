
use crate::mods::util::request::versatale::bearer_authentication;
use http_body_util::Full;
use hyper::body::Bytes;
use crate::mods::util::response::token::return_resource_object;
use hyper::{Request, Response};
use crate::mods::util::response::token::return_error_object;
pub async fn exchange_token(request: Request<hyper::body::Incoming>) -> Response<Full<Bytes>> {
    println!("exchanging token");
    if let Some(user_id) = bearer_authentication(&request) {
        return return_resource_object(user_id);
    } else {
        return_error_object(
            "Invalid_authorization_header".to_string(),
            "check yourauthorization header".to_string(),
        )
    }
}
