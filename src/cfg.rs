use std::collections::HashMap;
use std::convert::From;
use std::str::FromStr;
use std::{env, net, num};

use jsonwebtoken::{DecodingKey, EncodingKey};
use tracing::{debug, trace};

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
    pub address: net::IpAddr,
    pub port: u16,
    pub db_url: String,
    pub jwt_decoding_key: DecodingKey<'static>,
    pub jwt_encoding_key: EncodingKey,
}

// XXX: Maybe use failure here
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidEnvironment(env::VarError),
    InvalidAddress(net::AddrParseError),
    InvalidPort(num::ParseIntError),
}

impl Config {
    pub fn check_env() -> Result<(), String> {
        trace!("Config::check_env");

        let vars: HashMap<_, _> = env::vars().collect();
        for v in &REQUIRED_ENV_VARIABLES {
            if vars.contains_key(*v) && !vars[*v].is_empty() {
                debug!(
                    name = %v,
                    value = %vars[*v],
                    "Found required env variable"
                );
            } else {
                debug!(name = v, "Missing required env variable");
                return Err((*v).to_string());
            }
        }

        Ok(())
    }

    pub fn from_env() -> Result<Self, ParseError> {
        let address =
            net::IpAddr::from_str(&env::var("KAPITALIST_HOST").unwrap_or_else(|_| "::".into()))?;
        let port = env::var("KAPITALIST_PORT")
            .unwrap_or_else(|_| "5454".into())
            .parse()?;

        let jwt_secret = env::var("KAPITALIST_JWT_SECRET")?;
        let db_url = env::var("KAPITALIST_DB")?;
        Ok(Self {
            address,
            port,
            db_url,
            jwt_decoding_key: DecodingKey::from_secret(jwt_secret.as_ref()).into_static(),
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret.as_ref()),
        })
    }
}

impl From<env::VarError> for ParseError {
    fn from(error: env::VarError) -> Self {
        Self::InvalidEnvironment(error)
    }
}

impl From<net::AddrParseError> for ParseError {
    fn from(error: net::AddrParseError) -> Self {
        Self::InvalidAddress(error)
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(error: num::ParseIntError) -> Self {
        Self::InvalidPort(error)
    }
}
