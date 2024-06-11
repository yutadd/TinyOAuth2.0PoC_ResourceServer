use once_cell::sync::Lazy;
pub struct Endpoints {
    pub get_resource: String, // we only support read scope
}
pub struct Db {
    pub username: String,
    pub password: String,
    pub database: String,
    pub server_domain: String,
    pub server_port: u16,
    pub database_url:String
}
pub fn create_database_url(username: &str, password: &str, domain: &str, port: u16, database: &str) -> String {
    format!("mysql://{}:{}@{}:{}/{}", username, password, domain, port, database)
}
pub struct Config {
    pub server_address: String,
    pub self_server_port: u16,
    pub endpoints: Endpoints,
    pub db: Db,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let username = "root".to_string();
    let password = "P@ssw0rd".to_string();
    let database = "authorization_server".to_string();
    let server_domain = "mariadb".to_string();
    let server_port = 3306;
    print!("initializing config");
    Config {
    server_address: "localhost".to_string(),
    self_server_port: 8082,
    endpoints: Endpoints {
        get_resource: "/get/resource".to_string(),
    },
    db: Db {
        username: username.clone(),
        password: password.clone(),
        database: database.clone(),
        server_domain: server_domain.clone(),
        server_port: 3306,
        database_url: create_database_url(&username, &password, &server_domain, server_port, &database),
    },
}});
