use auth::JwtSecret;
use db::DatabaseExecutor;

use std::convert::From;
use std::{env, net};

use actix_web::actix::Addr;

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: net::SocketAddr,
    pub jwt_secret: JwtSecret,
    pub db_url: String,
}

// XXX: Maybe implement debug for this
pub struct AppState {
    pub(crate) config: Config,
    pub(crate) db: Addr<DatabaseExecutor>,
}

// XXX: Maybe use failure here
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidEnvironment(env::VarError),
    InvalidAddress(net::AddrParseError),
}

impl Config {
    pub fn from_env() -> Result<Config, ParseError> {
        let jwt_secret = env::var("KAPITALIST_JWT_SECRET")?;
        let addr = env::var("KAPITALIST_HOST")? + ":" + &env::var("KAPITALIST_PORT")?;
        let addr: net::SocketAddr = addr.parse()?;
        let db_url = env::var("KAPITALIST_DB")?;
        Ok(Config {
            addr: addr,
            jwt_secret: JwtSecret(jwt_secret),
            db_url: db_url,
        })
    }
}

impl AppState {
    pub fn new(cfg: Config, addr: Addr<DatabaseExecutor>) -> AppState {
        AppState {
            config: cfg,
            db: addr,
        }
    }
}

impl From<env::VarError> for ParseError {
    fn from(error: env::VarError) -> ParseError {
        ParseError::InvalidEnvironment(error)
    }
}

impl From<net::AddrParseError> for ParseError {
    fn from(error: net::AddrParseError) -> ParseError {
        ParseError::InvalidAddress(error)
    }
}
