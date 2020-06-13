/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Description |
/// | :--: | -- | -- | -- |
/// | POST | `/register` | `UserCreationRequest` | register new user |
/// | GET | `/me` | -- | get own user details |
/// | PUT | `/me` | `UserUpdateRequest` | update own user details |
/// | POST | `/token` | `TokenRequest` | obtain authentication token |
///
use std::sync::Arc;

use jsonwebtoken as jwt;
use tracing::{debug, instrument, trace, warn};
use warp::{reject, reply, Rejection, Reply};

use kapitalist_types::request::{TokenRequest, UserCreationRequest, UserUpdateRequest};
use kapitalist_types::response::TokenResponse;

use crate::auth::{TokenClaims, User};
use crate::db::{
    user::{GetUser, NewUser, UpdateUser},
    Database,
};
use crate::state::AppState;

#[instrument(skip(req, db))]
pub async fn register(db: Database, req: UserCreationRequest) -> Result<impl Reply, Rejection> {
    /* Register a new user
     *
     * - Check email is not registered yet
     * - Hash password
     * - Insert into DB
     * - Figure out what to return (redirect to me?)
     */
    trace!("register");
    let new_user = if let Some(u) = NewUser::from_request(req) {
        u
    } else {
        //return Err(super::util::bad_request("Password does not match criteria"));
        return Err(reject::reject());
    };
    match new_user.execute(&db.0) {
        Ok(Some(user)) => Ok(reply::json(&user.into_response())),
        Ok(None) => Err(reject::reject()),
        Err(err) => {
            debug!(error = %&err, "Error inserting user into database");
            Err(reject::reject())
        }
    }
}

#[instrument(skip(db, user))]
pub async fn get_me(db: Database, user: User) -> Result<impl Reply, Rejection> {
    trace!(ep = "user::get_me", "Endpoint called");
    let get_user = GetUser::ById(user.user_id);
    match get_user.execute(&db.0) {
        Ok(Some(user)) => Ok(reply::json(&user.into_response())),
        Ok(None) => {
            warn!(
                id = user.user_id,
                "Did not find authenticated user in database"
            );
            Err(reject::not_found())
        }
        Err(err) => {
            debug!(error = %&err, "Error getting user from database");
            Err(reject::reject())
        }
    }
}

#[instrument(skip(db, user))]
pub async fn put_me(
    db: Database,
    user: User,
    req: UserUpdateRequest,
) -> Result<impl Reply, Rejection> {
    trace!(ep = "user::put_me", "Endpoint called");
    if !req.is_valid() {
        // At least one field has to be set, could also return 301 unchanged?
        return Err(reject::reject());
    }

    let update_user = UpdateUser::from_request(user.user_id, req);
    match update_user.execute(&db.0) {
        // XXX: This return the user entity completely
        Ok(Some(user)) => Ok(reply::json(&user.into_response())),
        //Ok(None) => Err(reject::not_found(&"user")),
        Ok(None) => Err(reject::not_found()),
        Err(err) => {
            debug!(error = %&err, "Error updating user in database");
            Err(reject::reject())
        }
    }
}

#[instrument(skip(state, db))]
pub async fn token(
    state: Arc<AppState>,
    db: Database,
    req: TokenRequest,
) -> Result<impl Reply, Rejection> {
    /* Authenticate and request a token
     *
     * - Check email exists
     * - Verify password via libreauth::pass::HashBuilder
     * - Generate and return token
     */
    trace!(ep = "user::token", "Endpoint called");

    use libreauth::pass::HashBuilder;

    let get_user = GetUser::ByEmail(req.email.clone());
    match get_user.execute(&db.0) {
        Ok(Some(user)) => {
            // XXX: Should handle errors here as well
            let hasher =
                HashBuilder::from_phc(&user.secret).expect("[CRIT] Failed to create Hasher");
            if hasher.is_valid(&req.password) {
                // Password check succeeded -> Issuing token
                let claims = TokenClaims::new("auth", user.id);
                let jwt = jwt::encode(
                    &jwt::Header::default(),
                    &claims,
                    &state.config.jwt_encoding_key,
                )
                .expect("Failed to encode jwt token");

                Ok(reply::json(&TokenResponse { token: jwt }))
            } else {
                // Password check failed -> Return 401 - Unauthorized
                Err(reject::reject())
            }
        }
        // User entity was not found in database -> Return 401 to prevent information leakage
        Ok(None) => Err(reject::reject()),
        // There was an error contacting the db -> Log error and return 500
        Err(err) => {
            debug!(error = %&err, "Error loading user from database");
            Err(reject::reject())
        }
    }
}
