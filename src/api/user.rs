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
use actix_web::{
    http::StatusCode, AsyncResponder, HttpResponse, Json, Responder, State,
};
use futures::Future;

use auth::TokenClaims;
use db::{
    executor,
    user::{GetUser, NewUser},
};
use request::{TokenRequest, UserCreationRequest, UserUpdateRequest};
use response::TokenResponse;
use state::AppState;

pub fn register((state, data): (State<AppState>, Json<UserCreationRequest>)) -> impl Responder {
    /* Register a new user
     *
     * - Check email is not registered yet
     * - Hash password
     * - Insert into DB
     * - Figure out what to return (redirect to me?)
     */

    let new_user = NewUser::from_request(data.0);
    state
        .db
        .send(new_user)
        .and_then(|res| match res {
            Ok(user) => {
                let resp = HttpResponse::Ok().json(user);
                eprintln!("{:?}", resp);
                Ok(resp)
            }
            Err(e) => {
                eprintln!("{:?}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}
/*pub fn get_me(_db: DbConn, _user: UserGuard) -> Option<String> {
    // TODO: Figure out what to return here
    Some("GET /me".into())
}

pub fn put_me(_db: DbConn, _user: UserGuard, req: Json<UserUpdateRequest>) -> Result<(), Json<ErrorResponse>> {
    if req.email.is_none() && req.password.is_none() && req.name.is_none() {
        // At least one field has to be set, could also return 301 unchanged?
        return Err(Json(ErrorResponse::bad_request("Request has to contain at least one field to update")));
    }

    println!("PUT /me: email={:?}, password={:?}, name={:?}", req.email, req.password, req.name);
    Ok(())
}*/

pub fn token((state, data): (State<AppState>, Json<TokenRequest>)) -> impl Responder {
    /* Authenticate and request a token
     *
     * - Check email exists
     * - Verify password via scrypt_check
     * - Generate and return token
     */
    use pwhash::scrypt::scrypt_check;

    state.db
        .send(GetUser(data.email.clone()))
        .and_then(move |res| {
            match res {
                Ok(user) => {
                    if scrypt_check(&data.password, &user.secret_hash)
                        .expect("[CRIT] Found invalid hash in db")
                    {
                        // Password check succeeded -> Issuing token
                        let claims = TokenClaims::new("auth", user.id);
                        let jwt = ::jwt::encode(
                            &::jwt::Header::default(),
                            &claims,
                            state.config.jwt_secret.0.as_ref(),
                        ).unwrap();
                        eprintln!("{:?}", jwt);
                        Ok(HttpResponse::Ok().json(TokenResponse { token: jwt }))
                    } else {
                        // Password check failed -> Return 401 - Unauthorized
                        Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).json("Unauthorized"))
                    }
                }
                // XXX: Fix error type from DbExecutor and match here to differentiate between 4XX and 5XX errors
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}
