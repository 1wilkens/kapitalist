use actix_web::{
    error::{Error, ErrorUnauthorized},
    http::header::AUTHORIZATION,
    FromRequest, HttpRequest,
};
use chrono::serde::ts_seconds::{deserialize as from_ts, serialize as to_ts};
use chrono::{offset::FixedOffset, DateTime, Utc};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use slog::debug;

use crate::state::AppState;

#[derive(Debug, Clone)]
pub struct JwtSecret(pub String);

#[derive(Debug, Serialize, Deserialize)]
/// Represents claims included in kapitalist issued json web tokens
pub(crate) struct TokenClaims {
    // Issuer
    pub iss: String,
    // Subjec
    pub sub: String,
    // Audience
    pub aud: String,
    // Issued At
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    pub iat: DateTime<Utc>,
    // Expiration Time
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    pub exp: DateTime<Utc>,
    // User Id
    pub uid: i64,
}

impl TokenClaims {
    /// Create a new TokenClaims instance with the given subject and user is
    pub(crate) fn new(sub: &str, uid: i64) -> Self {
        // TODO: make this configurable and use real urls
        Self {
            iss: "kapitalist".into(),
            aud: "kapitalist".into(),
            sub: sub.into(),
            iat: Utc::now(),
            exp: Utc::now() + FixedOffset::east(12 * 3600),
            uid: uid,
        }
    }
}

/// Request guard which validates the user's token
pub struct UserGuard {
    pub user_id: i64, // TODO: Add more fields as required
}

impl FromRequest<AppState> for UserGuard {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<AppState>, _: &Self::Config) -> Self::Result {
        let secret = req.state().config.jwt_secret.0.as_ref();

        let headers = req.headers().get_all(AUTHORIZATION);
        for value in headers {
            // Skip invalid ASCII headers
            let value = match value.to_str() {
                Ok(v) => v,
                Err(_) => continue,
            };
            // Extract Bearer
            let parts: Vec<&str> = value.split(' ').collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                // We have a bearer token
                let validation = Validation {
                    leeway: 60,
                    ..Validation::default()
                };
                debug!(&req.state().log, "Validating bearer token"; "token" => &parts[1]);
                let token = match decode::<TokenClaims>(&parts[1], secret, &validation) {
                    Ok(token) => token,
                    Err(e) => {
                        // Print errors on debug output and continue to next token if any
                        debug!(&req.state().log, "Validation failed"; "error" => %e);
                        continue;
                    }
                };
                return Ok(Self {
                    user_id: token.claims.uid,
                });
            }
        }

        // XXX: Make this return a json error
        Err(ErrorUnauthorized("Unauthorized"))
    }
}
