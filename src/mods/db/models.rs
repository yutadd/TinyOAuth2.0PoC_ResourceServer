
pub struct User {
    pub id: String,
    pub authorization_code: Option<String>,
    pub authorization_code_expires_at: Option<i64>,
    pub access_token: Option<String>,
    pub access_token_expires_at: Option<i64>,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_at: Option<i64>,
    pub username: String,
    pub password: String,
    pub session_id: Option<String>,
    pub session_expires_at: Option<i64>,
}