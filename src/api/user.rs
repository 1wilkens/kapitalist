/* from doc/api.md
 *
 * ### User Management / Authentication
 * | Method | Endpoint | Payload/Params | Description |
 * | :--: | -- | -- | -- |
 * | POST | `/register` | UserCreationRequest | register new user |
 * | GET | `/me` | -- | get own user details |
 * | PUT | `/me` | UserUpdateRequest | update own user details |
 * |
 * | POST | `/auth` | TokenRequest | obtain authentication token |
 */
use actix_web::{AsyncResponder, Either, HttpResponse, Json, Responder, State};
use futures::Future;
use jsonwebtoken as jwt;
use slog::{debug, trace};

use crate::auth::{TokenClaims, UserGuard};
use crate::db::user::{GetUser, NewUser};
use crate::request::{TokenRequest, UserCreationRequest, UserUpdateRequest};
use crate::response::{ErrorResponse, TokenResponse};
use crate::state::AppState;

// TODO: Verify this use of Either
pub fn register((state, req): (State<AppState>, Json<UserCreationRequest>)) -> impl Responder {
    /* Register a new user
     *
     * - Check email is not registered yet
     * - Hash password
     * - Insert into DB
     * - Figure out what to return (redirect to me?)
     */
    trace!(&state.log, "Endpoint {ep} called",ep = "user::register"; "request" => ?&req.0);

    let new_user = match NewUser::from_request(req.0) {
        Some(u) => u,
        None => {
            return Either::A(HttpResponse::BadRequest().json(ErrorResponse::new("Password does not match criteria")))
        }
    };
    Either::B(
        state
            .db
            .send(new_user)
            .and_then(move |res| {
                let resp = match res {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => {
                        debug!(&state.log, "Error inserting user into database"; "error" => %&err);
                        HttpResponse::InternalServerError().into()
                    }
                };

                trace!(&state.log, "Endpoint {ep} returned",
                    ep = "user::register";
                    "response" => ?&resp.body(),
                    "statuscode" => %resp.status());
                Ok(resp)
            })
            .responder(),
    )
}
pub fn get_me((state, user): (State<AppState>, UserGuard)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "user::get_me");
    // TODO: Figure out what to return here
    Some(format!("GET /me (uid={})", user.user_id))
}

// XXX: This should probably return Result instead of Option
pub fn put_me((state, _user, req): (State<AppState>, UserGuard, Json<UserUpdateRequest>)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "user::put_me"; "request" => ?&req.0);

    if req.email.is_none() && req.password.is_none() && req.name.is_none() {
        // At least one field has to be set, could also return 301 unchanged?
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "Request has to contain at least one field to update",
        ));
    }

    HttpResponse::Ok().finish()
}

pub fn token((state, req): (State<AppState>, Json<TokenRequest>)) -> impl Responder {
    /* Authenticate and request a token
     *
     * - Check email exists
     * - Verify password via scrypt_check
     * - Generate and return token
     */
    use libreauth::pass::HashBuilder;
    trace!(&state.log, "Endpoint {ep} called", ep = "user::token"; "request" => ?&req.0);

    state
        .db
        .send(GetUser(req.email.clone()))
        .and_then(move |res| {
            let resp = match res {
                Ok(Some(user)) => {
                    // XXX: Should handle errors here as well
                    let hasher = HashBuilder::from_phc(&user.secret).expect("[CRIT] Failed to create Hasher");
                    if hasher.is_valid(&req.password) {
                        // Password check succeeded -> Issuing token
                        let claims = TokenClaims::new("auth", user.id);
                        let jwt = jwt::encode(&jwt::Header::default(), &claims, state.config.jwt_secret.0.as_ref())
                            .expect("Failed to encode jwt token");
                        let token = TokenResponse { token: jwt };

                        HttpResponse::Ok().json(token)
                    } else {
                        // Password check failed -> Return 401 - Unauthorized
                        HttpResponse::Unauthorized().json(ErrorResponse::unauthorized())
                    }
                }
                // User entity was not found in database -> Return 401 to prevent information leakage
                Ok(None) => HttpResponse::Unauthorized().json(ErrorResponse::unauthorized()),
                // There was an error contacting the db -> Log error and return 500
                Err(err) => {
                    debug!(&state.log, "Error loading user from database"; "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };

            trace!(&state.log, "Endpoint {ep} returned",
                ep = "user::token";
                "response" => ?&resp.body(),
                "statuscode" => %&resp.status());
            Ok(resp)
        })
        .responder()
}
