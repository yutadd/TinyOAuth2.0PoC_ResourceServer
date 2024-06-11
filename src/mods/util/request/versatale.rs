use hyper::Request;

use crate::mods::{config::config::CONFIG, db::models::User};

use crate::mods::db::repository::Repository;

use hyper::header::AUTHORIZATION;
pub fn bearer_authentication(request: &Request<hyper::body::Incoming>) -> Option<User> {
    println!("[util/req/vers] authenticating client");
    let repo = Repository::new(CONFIG.db.database_url.as_str()).expect("failed initialize repo");
    if let Some(auth_header) = request.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if let Ok(user)=repo.get_user_by_token(&token.to_string()){
                    return user;
                }else{
                    println!("[util/req/vers]failed to search user by token.")
                }
            }else{
                println!("[util/req/vers]error authentication method is not basic")
            }
        }else{
            println!("[util/req/vers]error converting to string.")
        }
    }else{
        println!("[util/req/vers]no authorization header found.")
    }
    None
}
