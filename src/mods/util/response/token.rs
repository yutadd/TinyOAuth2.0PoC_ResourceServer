use http_body_util::Full;
use hyper::Response;
use mysql::serde_json;
use hyper::body::Bytes;

use crate::mods::db::models::User;
pub fn return_resource_object(user:User) -> Response<Full<Bytes>> {
    let token_object = serde_json::json!({
        "username": user.username,
        "user_id": user.id,
    });
    let body = Full::from(Bytes::from(token_object.to_string()));
    Response::builder()
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}
pub fn return_error_object(error:String,error_description:String) -> Response<Full<Bytes>> {
    let error_object = serde_json::json!({
        "error": error,
        "error_description": error_description
    });
    let body = Full::from(Bytes::from(error_object.to_string()));
    Response::builder()
        .header("Content-Type", "application/json")
        .status(400)
        .body(body)
        .unwrap()
}
