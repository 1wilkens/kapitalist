use auth::JwtSecret;
use db::DatabaseExecutor;

use std::env;

use actix_web::actix::Addr;

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) jwt_secret: JwtSecret,
}

impl Config {
    pub fn from_env() -> Option<Config> {
        let jwt_secret = env::var("KAPITALIST_JWT_SECRET").ok()?;
        Some(Config{
            jwt_secret: JwtSecret(jwt_secret)
        })
    }
}

// XXX: Maybe implement debug for this?
pub struct AppState {
    pub(crate) config: Config,
    pub(crate) db: Addr<DatabaseExecutor>,
}

impl AppState {
    pub fn new(cfg: Config, addr: Addr<DatabaseExecutor>) -> AppState {
        AppState {
            config: cfg,
            db: addr,
        }
    }
}
