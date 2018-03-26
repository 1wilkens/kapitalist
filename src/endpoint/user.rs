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
use diesel;
use diesel::prelude::*;

use rocket::State;
use rocket::response::status;
use rocket_contrib::Json;

use auth::{JwtSecret, TokenClaims, UserGuard};
use db::DbConn;
use model::{User, NewUser};
use request::*;
use response::*;

#[post("/register", data = "<req>")]
pub fn register(db: DbConn, req: Json<UserCreationRequest>)
    -> Result<status::Created<Json<User>>, Json<ErrorResponse>> {
    /* Register a new user
     *
     * - Check email is not registered yet
     * - Hash password
     * - Insert into DB
     * - Figure out what to return (redirect to me?)
     */
    use schema::users::dsl::*;

    let exists = diesel::select(diesel::dsl::exists(users
        .filter(email.eq(&req.email))))
        .get_result(&*db)
        // TODO: expose error here in debug mode?
        .map_err(|_| Json(ErrorResponse::server_error()))?;

    if exists {
        // TODO: should we really return this message?
        return Err(Json(ErrorResponse::new(401, Some("User already exists"))));
    }

    // TODO: If this fails, scrypt failed (which should not happen?) investigate this some more
    let new_user = NewUser::from_request(req.0)
        .map_err(|_| Json(ErrorResponse::bad_request("Invalid body data")))?;
    let user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&*db)
        .map_err(|_| Json(ErrorResponse::server_error()))?;
    Ok(status::Created("/me".into(), Some(Json(user))))
}

#[get("/me")]
pub fn get_me(_db: DbConn, _user: UserGuard) -> Option<String> {
    // TODO: Figure out what to return here
    Some("GET /me".into())
}

#[put("/me", data = "<req>")]
pub fn put_me(_db: DbConn, _user: UserGuard, req: Json<UserUpdateRequest>) -> Result<(), Json<ErrorResponse>> {
    if req.email.is_none() && req.password.is_none() && req.name.is_none() {
        // At least one field has to be set, could also return 301 unchanged?
        return Err(Json(ErrorResponse::bad_request("Request has to contain at least one field to update")));
    }

    println!("PUT /me: email={:?}, password={:?}, name={:?}", req.email, req.password, req.name);
    Ok(())
}

#[post("/token", data = "<req>")]
pub fn token(db: DbConn, jwt: State<JwtSecret>, req: Json<TokenRequest>) -> Result<Json<TokenResponse>, Json<ErrorResponse>> {
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
}
