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

use rocket::response::status::{NotFound, Created};
use rocket_contrib::Json;

use auth::{UserGuard, TokenClaims};
use db::DbConn;
use model::*;
use request::*;
use response::*;

#[post("/register", data = "<req>")]
pub fn register(db: DbConn, req: Json<UserCreationRequest>)
    -> Result<Created<Json<User>>, Json<RequestError>> {
    /* Register a new user
     *
     * - Check email is not registered yet
     * - Hash password
     * - Insert into DB
     * - Figure out what to return (redirect to me?)
     */
    use schema::users::dsl::*;

    /*let params = get_scrypt_params();
    let hashed = scrypt_simple(&req.password, &params).expect("scrypt failed");*/

    let exists = diesel::select(diesel::dsl::exists(users
        .filter(email.eq(&req.email))))
        .get_result(&*db)
        .map_err(|err| Json(RequestError { code: 401, text: err.to_string() }))?;

    if exists {
        return Err(Json(RequestError { code: 401, text: "user already exists".into() }))
    }

    let new_user = NewUser::from_request(req.0).expect("failed to parse newuser");
    let user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&*db)
        .expect("query failed");
    Ok(Created("/me".into(), Some(Json(user))))
}

#[get("/me")]
pub fn get_me(_db: DbConn, _user: UserGuard) -> &'static str {
    // TODO: Implement this, when tokens are finalized
    "GET /me"
}

#[put("/me", data = "<req>")]
pub fn put_me(_db: DbConn, req: Json<UserUpdateRequest>) -> String {
    if req.email.is_none() && req.password.is_none() && req.name.is_none() {
        // At least one field has to be set, could also return 301 unchanged?
        return "400".into()
    }


    format!("PUT /me: email={:?}, password={:?}, name={:?}", req.email, req.password, req.name)
}

#[post("/token", data = "<req>")]
pub fn token(db: DbConn, req: Json<TokenRequest>) -> Result<Json<TokenResponse>, NotFound<String>> {
    /* Authenticate and request a token
     *
     * - Check email exists
     * - Verify password via scrypt_check
     * - Generate and return token
     */
    use pwhash::scrypt::scrypt_check;
    use schema::users::dsl::*;

    // TODO: handle failure gracefully
    let user = users
        .filter(email.eq(&req.email))
        .get_result::<User>(&*db)
        .expect("query failed");

    let res = scrypt_check(&req.password, &user.secret_hash).expect("invalid hash in db");
    if res {
        let claims = TokenClaims::new("auth", user.id);
        let jwt = ::jwt::encode(&::jwt::Header::default(), &claims, "supersecretkeyy".as_ref()).unwrap();
        Ok(Json(TokenResponse { token: jwt }))
    } else {
        Err(NotFound("fail".into()))
    }
}
