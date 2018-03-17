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

use db::DbConn;
use rocket_contrib::Json;
use pwhash::scrypt::{ScryptParams, scrypt_simple};

#[derive(Deserialize)]
pub struct UserCreationRequest {
    email:    String,
    password: String,
    name:     String,
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    email:    Option<String>,
    password: Option<String>,
    name:     Option<String>,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    email:    String,
    password: String,
}

#[post("/register", data = "<req>")]
pub fn register(_db: DbConn, req: Json<UserCreationRequest>) -> String {
    // See: https://blog.filippo.io/the-scrypt-parameters/ for the choice of parameters
    let params = ScryptParams::new(15, 8, 1);
    let hashed = scrypt_simple(&req.password, &params);
    format!("POST /register: email={}, password={}, name={}, hashed={:?}", req.email, req.password, req.name, hashed)
}

#[get("/me")]
pub fn get_me(_db: DbConn) -> &'static str {
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

#[post("/authenticate", data = "<req>")]
pub fn authenticate(_db: DbConn, req: Json<TokenRequest>) -> String {
    let params = ScryptParams::new(15, 8, 1);
    let hashed = scrypt_simple(&req.password, &params);
    format!("POST /authenticate: email={}, password={}, hashed={:?}", req.email, req.password, hashed) 
}