use auth::JwtSecret;
use db::DatabaseExecutor;

use std::convert::From;
use std::{env, net};

use actix_web::actix;
use slog;
use slog_stdlog;

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: net::SocketAddr,
    pub jwt_secret: JwtSecret,
    pub db_url: String,
}

// XXX: Maybe implement debug for this
pub struct AppState {
    pub(crate) log: slog::Logger,
    pub(crate) config: Config,
    pub(crate) db: actix::Addr<DatabaseExecutor>,
}

pub struct AppStateBuilder {
    pub(crate) config: Config,
    pub(crate) db: actix::Addr<DatabaseExecutor>,
    pub(crate) log: Option<slog::Logger>,
}

// XXX: Maybe use failure here
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidEnvironment(env::VarError),
    InvalidAddress(net::AddrParseError),
}

impl Config {
    pub fn from_env() -> Result<Config, ParseError> {
        let addr = env::var("KAPITALIST_HOST")? + ":" + &env::var("KAPITALIST_PORT")?;
        let addr: net::SocketAddr = addr.parse()?;
        let jwt_secret = env::var("KAPITALIST_JWT_SECRET")?;
        let db_url = env::var("KAPITALIST_DB")?;
        Ok(Config {
            addr: addr,
            jwt_secret: JwtSecret(jwt_secret),
            db_url: db_url,
        })
    }
}

impl AppState {
    pub fn new(cfg: Config, addr: actix::Addr<DatabaseExecutor>) -> AppStateBuilder {
        AppStateBuilder {
            config: cfg,
            db: addr,
            log: None
        }
    }
}

impl AppStateBuilder {
    pub fn with_logger(mut self, log: slog::Logger) -> Self {
        self.log = Some(log);
        self
    }

    pub fn build(self) -> AppState {
        use slog::Drain;

        let log = self.log.unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!()));
        AppState {
            log: log,
            config: self.config,
            db: self.db,
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
