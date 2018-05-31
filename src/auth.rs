use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds::serialize as to_ts;
use chrono::serde::ts_seconds::deserialize as from_ts;

use jwt::{decode, Validation};
use rocket::Outcome;
use rocket::State;
use rocket::request::{self, Request, FromRequest};

pub struct JwtSecret(pub String);

#[derive(Debug, Serialize, Deserialize)]
/// Represents claims included in kapitalist issued json web tokens
pub struct TokenClaims {
    // Issuer
    pub iss:    String,
    // Subjec
    pub sub:    String,
    // Audience
    pub aud:    String,
    // Issued At
    #[serde(serialize_with="to_ts", deserialize_with="from_ts")]
    pub iat:    DateTime<Utc>,
    // Expiration Time
    #[serde(serialize_with="to_ts", deserialize_with="from_ts")]
    pub exp:    DateTime<Utc>,
    // User Id
    pub uid:   i32,
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
            uid: uid
        }
    }
}

/// Request guard which checks
pub struct UserGuard {
    // TODO: Add more fields as required
    user_id: i32
}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<UserGuard, ()> {
        let jwt = req.guard::<State<JwtSecret>>()?;
        let headers = req.headers().get("Authorization");
        for h in headers {
            let parts: Vec<&str> = h.split(' ').collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                // We have a bearer token
                let mut validation = Validation { leeway: 60, ..Default::default()};
                let token = match decode::<TokenClaims>(&parts[1], jwt.0.as_ref(), &validation) {
                    Ok(token) => token,
                    Err(_)    => continue
                };
                return Outcome::Success(UserGuard { user_id: token.claims.uid });
            }
        }

        // TODO: Figure out what makes more sense here
        info_!("Forwarding because of missing or invalid Authorization header");
        Outcome::Forward(())
        //Outcome::Failure((Status::Unauthorized, ()))
    }
}
