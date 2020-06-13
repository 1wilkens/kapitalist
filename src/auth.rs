use std::sync::Arc;

use chrono::serde::ts_seconds::{deserialize as from_ts, serialize as to_ts};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use tracing::debug;
use warp::{
    reject::{self, Rejection},
    Filter,
};

use crate::state;

pub fn check(
    st: Arc<state::AppState>,
) -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    warp::header("Authorization")
        .and(state::attach(st.clone()))
        .and_then(check_token)
}

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
    /// Create a new `TokenClaims` instance with the given subject and user is
    pub(crate) fn new(sub: &str, uid: i64) -> Self {
        // TODO: make this configurable and use real urls
        Self {
            iss: "kapitalist".into(),
            aud: "kapitalist".into(),
            sub: sub.into(),
            iat: Utc::now(),
            /// XXX: Make this configurable
            exp: Utc::now() + Duration::seconds(7 * 24 * 3600),
            uid: uid,
        }
    }
}

/// Request guard which validates the user's token
#[derive(Debug, Clone, Copy)]
pub struct User {
    pub user_id: i64, // TODO: Add more fields as required
}

pub async fn check_token(header: String, state: Arc<state::AppState>) -> Result<User, Rejection> {
    let state = state.clone();
    let key = &state.config.jwt_decoding_key;

    debug!(%header, "check_auth");
    // Extract Bearer
    let parts: Vec<&str> = header.split(' ').collect();
    if parts.len() == 2 && parts[0] == "Bearer" {
        // We have a bearer token
        let validation = Validation {
            leeway: 60,
            ..Validation::default()
        };
        debug!(token = %parts[1], "Validating bearer token");
        let token = match decode::<TokenClaims>(&parts[1], key, &validation) {
            Ok(token) => token,
            Err(e) => {
                // Print errors on debug output and continue to next token if any
                debug!(error = %e, "Validation failed");
                // FIXME: return 401
                return Err(reject::reject());
            }
        };

        return Ok(User {
            user_id: token.claims.uid,
        });
    }

    // FIXME: return 401
    Err(reject::reject())
}
