use auth::JwtSecret;
use db::DatabaseExecutor;

use actix_web::actix::Addr;

pub struct AppState {
    pub(crate) config: Config,
    pub(crate) db: Addr<DatabaseExecutor>,
}

impl AppState {
    pub fn new(addr: Addr<DatabaseExecutor>) -> AppState {
        AppState {
            config: Config {
                jwt_secret: JwtSecret(String::new()),
            },
            db: addr,
        }
    }
}

pub struct Config {
    pub(crate) jwt_secret: JwtSecret,
}
