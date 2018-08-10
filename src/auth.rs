use actix_web::{
    error::{Error, ErrorUnauthorized},
    http::header::AUTHORIZATION,
    FromRequest, HttpRequest,
};
use chrono::serde::ts_seconds::{deserialize as from_ts, serialize as to_ts};
use chrono::{DateTime, Utc};
use jwt::{decode, Validation};

use state::AppState;

#[derive(Debug)]
pub struct JwtSecret(pub String);

#[derive(Debug, Serialize, Deserialize)]
/// Represents claims included in kapitalist issued json web tokens
pub struct TokenClaims {
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
    pub uid: i32,
}

impl TokenClaims {
    /// Create a new TokenClaims instance with the given subject and user is
    pub fn new(sub: &str, uid: i32) -> TokenClaims {
        // TODO: make this configurable and use real urls
        TokenClaims {
            iss: "kapitalist".into(),
            aud: "kapitalist".into(),
            sub: sub.into(),
            iat: Utc::now(),
            exp: Utc::now(),
            uid: uid,
        }
    }
}

/// Request guard which validates the user's token
pub struct UserGuard {
    pub user_id: i32, // TODO: Add more fields as required
}

impl FromRequest<AppState> for UserGuard {
    type Config = ();
    type Result = Result<UserGuard, Error>;

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
                let mut validation = Validation {
                    leeway: 60,
                    ..Default::default()
                };
                let token = match decode::<TokenClaims>(&parts[1], secret, &validation) {
                    Ok(token) => token,
                    Err(_) => continue,
                };
                return Ok(UserGuard {
                    user_id: token.claims.uid,
                });
            }
        }

        // XXX: Make this return a json error
        Err(ErrorUnauthorized("Unauthorized"))
    }
}
