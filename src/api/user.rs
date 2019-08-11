/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Description |
/// | :--: | -- | -- | -- |
/// | POST | `/register` | `UserCreationRequest` | register new user |
/// | GET | `/me` | -- | get own user details |
/// | PUT | `/me` | `UserUpdateRequest` | update own user details |
/// | POST | `/token` | `TokenRequest` | obtain authentication token |
///
use jsonwebtoken as jwt;
use rocket::State;
use rocket_contrib::json::Json;
use slog::{debug, trace, warn};

use kapitalist_types::request::{TokenRequest, UserCreationRequest, UserUpdateRequest};
use kapitalist_types::response::{TokenResponse, UserResponse};

use crate::auth::{TokenClaims, User};
use crate::db::{
    user::{GetUser, NewUser, UpdateUser},
    Database,
};
use crate::state::AppState;

#[post("/register", data = "<req>")]
pub fn register(
    state: State<AppState>,
    db: Database,
    req: Json<UserCreationRequest>,
) -> super::Result<Json<UserResponse>> {
    /* Register a new user
     *
     * - Check email is not registered yet
     * - Hash password
     * - Insert into DB
     * - Figure out what to return (redirect to me?)
     */
    let new_user = if let Some(u) = NewUser::from_request(req.0) {
        u
    } else {
        return Err(super::util::bad_request("Password does not match criteria"));
    };
    match new_user.execute(&*db) {
        Ok(Some(user)) => Ok(Json(user.into_response())),
        Ok(None) => Err(super::util::unauthorized()),
        Err(err) => {
            debug!(&state.log, "Error inserting user into database"; "error" => %&err);
            Err(super::util::internal_server_error())
        }
    }
}

#[get("/me")]
pub fn get_me(user: User, state: State<AppState>, db: Database) -> super::Result<Json<UserResponse>> {
    trace!(&state.log, "Endpoint {ep} called", ep = "user::get_me");
    let get_user = GetUser::by_id(user.user_id);
    match get_user.execute(&*db) {
        Ok(Some(user)) => Ok(Json(user.into_response())),
        Ok(None) => {
            warn!(
                &state.log,
                "Did not find authenticated user in database: {id}",
                id = user.user_id
            );
            Err(super::util::not_found("user"))
        }
        Err(err) => {
            debug!(state.log, "Error getting user from database"; "error" => %&err);
            Err(super::util::internal_server_error())
        }
    }
}

#[put("/me", data = "<req>")]
pub fn put_me(
    user: User,
    state: State<AppState>,
    db: Database,
    req: Json<UserUpdateRequest>,
) -> super::Result<Json<UserResponse>> {
    if !req.is_valid() {
        // At least one field has to be set, could also return 301 unchanged?
        return Err(super::util::update_request_invalid());
    }

    let update_user = UpdateUser::from_request(user.user_id, req.0);
    match update_user.execute(&*db) {
        // XXX: This return the user entity completely
        Ok(Some(user)) => Ok(Json(user.into_response())),
        Ok(None) => Err(super::util::not_found(&"user")),
        Err(err) => {
            debug!(&state.log, "Error updating user in database";
                        "error" => %&err);
            Err(super::util::internal_server_error())
        }
    }
}

#[post("/token", data = "<req>")]
pub fn token(state: State<AppState>, db: Database, req: Json<TokenRequest>) -> super::Result<Json<TokenResponse>> {
    /* Authenticate and request a token
     *
     * - Check email exists
     * - Verify password via scrypt_check
     * - Generate and return token
     */
    use libreauth::pass::HashBuilder;

    let get_user = GetUser::by_email(req.email.clone());
    match get_user.execute(&*db) {
        Ok(Some(user)) => {
            // XXX: Should handle errors here as well
            let hasher = HashBuilder::from_phc(&user.secret).expect("[CRIT] Failed to create Hasher");
            if hasher.is_valid(&req.password) {
                // Password check succeeded -> Issuing token
                let claims = TokenClaims::new("auth", user.id);
                let jwt = jwt::encode(&jwt::Header::default(), &claims, state.config.jwt_secret.0.as_ref())
                    .expect("Failed to encode jwt token");

                Ok(Json(TokenResponse { token: jwt }))
            } else {
                // Password check failed -> Return 401 - Unauthorized
                Err(super::util::unauthorized())
            }
        }
        // User entity was not found in database -> Return 401 to prevent information leakage
        Ok(None) => Err(super::util::unauthorized()),
        // There was an error contacting the db -> Log error and return 500
        Err(err) => {
            debug!(&state.log, "Error loading user from database"; "error" => %&err);
            Err(super::util::internal_server_error())
        }
    }
}
