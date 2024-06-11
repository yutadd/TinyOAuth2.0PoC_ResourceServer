use crate::mods::db::models:: User;
use mysql::prelude::*;
use mysql::*;

use crate::mods::db::repository::params;

pub struct Repository {
    pool: Pool,
}
impl Repository {
    pub fn new(database_url: &str) -> Result<Self, mysql::Error> {
        let pool = Pool::new(Opts::from_url(database_url).expect("Invalid DB URL"))?;
        Ok(Repository { pool })
    }
pub fn get_user_by_token(&self, token: &String) -> Result<Option<User>, mysql::Error> {
    let mut conn = self.pool.get_conn()?;
    println!("[db/repository]searching user by token:{}",token);
    let user: Option<User> = conn.exec_first(
        "SELECT id, authorization_code, authorization_code_expires_at, access_token, access_token_expires_at, refresh_token, refresh_token_expires_at, username, password, session_id, session_expires_at FROM users WHERE access_token = :token OR refresh_token = :token",
        params! {
            "token" => token,
        },
    )?.map(|(id, authorization_code, authorization_code_expires_at, access_token, access_token_expires_at, refresh_token, refresh_token_expires_at, username, password, session_id, session_expires_at): (String, Option<String>, Option<i64>, Option<String>, Option<i64>, Option<String>, Option<i64>, String, String, Option<String>, Option<i64>)| {
        User {
            id,
            authorization_code,
            authorization_code_expires_at,
            access_token,
            access_token_expires_at,
            refresh_token,
            refresh_token_expires_at,
            username,
            password,
            session_id,
            session_expires_at,
        }
    });
    println!("[db/repository]search result:{}",user.is_some());
    Ok(user)
}

}

