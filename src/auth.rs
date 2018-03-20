use chrono::{DateTime, Utc};
use jwt::{decode, Algorithm, Validation};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};


#[derive(Debug, Serialize, Deserialize)]
/// Represents claims included in kapitalist issued json web tokens
pub struct TokenClaims {
    // Issuer
    pub iss:    String,
    // Subjec
    pub sub:    String,
    // Issued At
    pub iat:    DateTime<Utc>,
    // Expiration Time
    pub exp:    DateTime<Utc>,
    // User Id
    pub uid:   i32,
}

impl TokenClaims {
    /// new blabla
    pub fn new(sub: &str, uid: i32) -> TokenClaims {
        TokenClaims {
            iss: "kapitalist".into(),
            sub: sub.into(),
            iat: Utc::now(),
            exp: Utc::now(),
            uid: uid
        }
    }
}

pub struct UserGuard {
    user_id: i32
}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<UserGuard, ()> {
        let headers = req.headers().get("Authorization");
        for h in headers {
            let parts: Vec<&str> = h.split(' ').collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                // We have a bearer token
                let mut validation = Validation { leeway: 60, ..Default::default()};
                // TODO: date validation is currently broken, reenable when upstream is fixed
                validation.validate_iat = false;
                validation.validate_exp = false;
                // TODO: load secret from env (via env! macro?)
                let token = match decode::<TokenClaims>(&parts[1],
                    b"supersecretkeyy", &validation) {
                    Ok(token) => token,
                    Err(_)    => return Outcome::Failure((Status::Unauthorized, ()))
                };
                return Outcome::Success(UserGuard { user_id: token.claims.uid });
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}