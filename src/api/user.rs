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
use actix_web::{http::StatusCode, AsyncResponder, Either, HttpResponse, Json, Responder, State};
use futures::Future;

use auth::{TokenClaims, UserGuard};
use db::user::{GetUser, NewUser};
use request::{TokenRequest, UserCreationRequest, UserUpdateRequest};
use response::{ErrorResponse, TokenResponse};
use state::AppState;

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
            return Either::A(
                HttpResponse::BadRequest()
                    .json(ErrorResponse::new("Password does not match criteria")),
            )
        }
    };
    Either::B(
        state
            .db
            .send(new_user)
            .and_then(move |res| match res {
                Ok(user) => {
                    trace!(&state.log, "Endpoint {ep} returned", ep = "user::register";
                        "response" => ?&user,
                        "statuscode" => 200);
                    Ok(HttpResponse::Ok().json(user))
                }
                Err(e) => {
                    trace!(&state.log, "Endpoint {ep} returned", ep = "user::register";
                        "error" => ?e,
                        "statuscode" => 500);
                    Ok(HttpResponse::InternalServerError().into())
                }
            }).responder(),
    )
}
pub fn get_me((state, user): (State<AppState>, UserGuard)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "user::get_me");
    // TODO: Figure out what to return here
    Some(format!("GET /me (uid={})", user.user_id))
}

// XXX: This should probably return Result instead of Option
pub fn put_me(
    (state, _user, req): (State<AppState>, UserGuard, Json<UserUpdateRequest>),
) -> impl Responder {
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
            match res {
                Ok(user) => {
                    // XXX: Should handle errors here as well
                    let hasher = HashBuilder::from_phc(&user.secret)
                        .expect("[CRIT] Failed to create Hasher");
                    if hasher.is_valid(&req.password) {
                        // Password check succeeded -> Issuing token
                        let claims = TokenClaims::new("auth", user.id);
                        let jwt = ::jwt::encode(
                            &::jwt::Header::default(),
                            &claims,
                            state.config.jwt_secret.0.as_ref(),
                        ).expect("Failed to encode jwt token");
                        let token = TokenResponse { token: jwt };

                        trace!(&state.log, "Endpoint {ep} returned",
                            ep = "user::token";
                            "response" => ?&token,
                            "statuscode" => 200);
                        Ok(HttpResponse::Ok().json(token))
                    } else {
                        // Password check failed -> Return 401 - Unauthorized

                        trace!(&state.log, "Endpoint {ep} returned",
                            ep = "user::token";
                            "statuscode" => 401);
                        Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).json("Unauthorized"))
                    }
                }
                // XXX: Fix error type from DbExecutor and match here to differentiate between 4XX and 5XX errors
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}
