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
use actix_web::{AsyncResponder, Responder, HttpRequest, HttpResponse, Json};
use futures::Future;

//use auth::{JwtSecret, TokenClaims, UserGuard};
use db::model::NewUser;
use request::*;
use state::AppState;

pub fn register((req, data): (HttpRequest<AppState>, Json<UserCreationRequest>)) -> impl Responder {
    /* Register a new user
     *
     * - Check email is not registered yet
     * - Hash password
     * - Insert into DB
     * - Figure out what to return (redirect to me?)
     */

    let new_user = NewUser::from_request(data.0);
    req.state().db.send(new_user)
        .and_then(|res| {
            match res {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(_)  => Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
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
}

pub fn token(db: DbConn, jwt: JwtSecret, req: Json<TokenRequest>) -> Result<Json<TokenResponse>, Json<ErrorResponse>> {
    /* Authenticate and request a token
     *
     * - Check email exists
     * - Verify password via scrypt_check
     * - Generate and return token
     */
    use pwhash::scrypt::scrypt_check;
    use schema::users::dsl::*;

    let user = users
        .filter(email.eq(&req.email))
        .get_result::<User>(&*db)
        .map_err(|_| Json(ErrorResponse::server_error()))?;

    if scrypt_check(&req.password, &user.secret_hash).expect("[CRIT] Found invalid hash in db") {
        // Password check succeeded -> Issuing token
        let claims = TokenClaims::new("auth", user.id);
        let jwt = ::jwt::encode(&::jwt::Header::default(), &claims, jwt.0.as_ref()).unwrap();
        Ok(Json(TokenResponse { token: jwt }))
    } else {
        // Password check failed -> Return 401 - Unauthorized
        Err(Json(ErrorResponse::new(401, Some("Unauthorized"))))
    }
}*/
