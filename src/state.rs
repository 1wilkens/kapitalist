use slog::{self, debug, o, trace};
use slog_stdlog;

use std::collections::HashMap;
use std::convert::From;
use std::{env, net};

use crate::auth::JwtSecret;

/// Required environment variables for kapitalist
#[allow(unused_doc_comments)]
static REQUIRED_ENV_VARIABLES: [&str; 4] = [
    /// Which IP address to listen on
    "KAPITALIST_HOST",
    /// Which port to listen on
    "KAPITALIST_PORT",
    /// Connection string of the backing database (diesel format)
    "KAPITALIST_DB",
    /// JWT secret to sign tokens with
    "KAPITALIST_JWT_SECRET",
];

#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub db_url: String,
    pub jwt_secret: JwtSecret,
}

// XXX: Maybe implement debug for this
pub struct AppState {
    pub(crate) config: Config,
    pub(crate) log: slog::Logger,
}

pub struct AppStateBuilder {
    pub(crate) config: Config,
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

        for v in &REQUIRED_ENV_VARIABLES {
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
    pub fn from_env() -> Result<Self, ParseError> {
        let address = env::var("KAPITALIST_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = env::var("KAPITALIST_PORT").unwrap_or_else(|_| "5454".into()).parse().unwrap();

        let jwt_secret = env::var("KAPITALIST_JWT_SECRET")?;
        let db_url = env::var("KAPITALIST_DB")?;
        Ok(Self {
            address,
            port,
            db_url,
            jwt_secret: JwtSecret(jwt_secret),
        })
    }
}

impl AppState {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(cfg: Config) -> AppStateBuilder {
        AppStateBuilder {
            config: cfg,
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
        }
    }
}

impl From<env::VarError> for ParseError {
    fn from(error: env::VarError) -> Self {
        ParseError::InvalidEnvironment(error)
    }
}

impl From<net::AddrParseError> for ParseError {
    fn from(error: net::AddrParseError) -> Self {
        ParseError::InvalidAddress(error)
    }
}
