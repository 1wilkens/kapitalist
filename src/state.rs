use actix_web::actix;
use slog::{self, debug, o, trace};
use slog_stdlog;

use std::collections::HashMap;
use std::convert::From;
use std::{env, net};

use crate::auth::JwtSecret;
use crate::db::DatabaseExecutor;

/// Required environment variables for kapitalist
///
/// - KAPITALIST_HOST       - Which IP address to listen on
/// - KAPITALIST_PORT       - Which port to listen on
/// - KAPITALIST_DB         - Connection string of the backing database (diesel format)
/// - KAPITALIST_JWT_SECRET - JWT secret to sign tokens with
static REQUIRED_ENV_VARIABLES: [&str; 4] = [
    "KAPITALIST_HOST",
    "KAPITALIST_PORT",
    "KAPITALIST_DB",
    "KAPITALIST_JWT_SECRET",
];

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: net::SocketAddr,
    pub db_url: String,
    pub jwt_secret: JwtSecret,
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
    pub fn check_env(log: &slog::Logger) -> Result<(), String> {
        trace!(&log, "Checking environment");
        let vars: HashMap<String, String> = env::vars().collect();

        for v in REQUIRED_ENV_VARIABLES.iter() {
            if vars.contains_key(*v) && !vars[*v].is_empty() {
                debug!(&log, "Found required env variable";
                "value" => &vars[*v],
                "name" => v);
            } else {
                debug!(&log, "Missing required env variables"; "name" => v);
                return Err(v.to_string());
            }
        }

        Ok(())
    }
    pub fn from_env() -> Result<Config, ParseError> {
        let ip = env::var("KAPITALIST_HOST").unwrap_or("0.0.0.0".into());
        let port = env::var("KAPITALIST_PORT").unwrap_or("5454".into());
        let addr = ip + ":" + &port;
        let addr: net::SocketAddr = addr.parse()?;
        let jwt_secret = env::var("KAPITALIST_JWT_SECRET")?;
        let db_url = env::var("KAPITALIST_DB")?;
        Ok(Config {
            addr: addr,
            db_url: db_url,
            jwt_secret: JwtSecret(jwt_secret),
        })
    }
}

impl AppState {
    pub fn new(cfg: Config, addr: actix::Addr<DatabaseExecutor>) -> AppStateBuilder {
        AppStateBuilder {
            config: cfg,
            db: addr,
            log: None,
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

        let log = self
            .log
            .unwrap_or_else(|| slog::Logger::root(slog_stdlog::StdLog.fuse(), o!()));
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
