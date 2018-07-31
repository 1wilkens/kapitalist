use auth::JwtSecret;

pub struct AppState {
    pub config: Config,
}

impl Default for AppState {
    fn default() -> Self {
        AppState { config: Config { jwt_secret: JwtSecret(String::new()) } }
    }
}

pub struct Config {
    pub(crate) jwt_secret: JwtSecret,
}
