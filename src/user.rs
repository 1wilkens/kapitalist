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

use models::*;

use db::DbConn;
use diesel;
use diesel::prelude::*;
use pwhash::scrypt::{ScryptParams, scrypt_check}; //, scrypt_simple};
use rocket::response::status::NotFound;
use rocket_contrib::Json;

#[post("/register", data = "<req>")]
pub fn register(db: DbConn, req: Json<UserCreationRequest>) -> Result<Json<User>, NotFound<String>> {
    /* Register a new user
     *
     * - Check email is not registered yet
     * - Hash password
     * - Insert into DB
     * - Figure out what to return (redirect to me?)
     */
    use schema::users;

    /*let params = get_scrypt_params();
    let hashed = scrypt_simple(&req.password, &params).expect("scrypt failed");*/

    let new_user = NewUser::from_request(req.0).expect("failed to parse newuser");
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&*db)
        .expect("query failed");
    Ok(Json(user))
}

#[get("/me")]
pub fn get_me(_db: DbConn) -> &'static str {
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
pub fn token(db: DbConn, req: Json<TokenRequest>) -> Result<String, NotFound<String>> {
    /* Authenticate and request a token
     *
     * - Check email exists
     * - Verify password via scrypt_check
     * - Generate and return token
     */
    use schema::users::dsl::*;

    // TODO: handle failure gracefully
    let user = users
        .filter(email.eq(&req.email))
        .get_result::<User>(&*db)
        .expect("query failed");

    let res = scrypt_check(&req.password, &user.secret_hash).expect("invalid hash in db");
    if res {
        let claims = TokenClaims {
            iss: "kapitalist".into(),
            user: user.id
        };
        let jwt = ::jwt::encode(&::jwt::Header::default(), &claims, "supersecretkey".as_ref()).unwrap();
        Ok(jwt)
    } else {
        Err(NotFound("fail".into()))
    }
}

fn get_scrypt_params() -> ScryptParams {
    /* See: https://blog.filippo.io/the-scrypt-parameters/ for the choice of parameters
     *
     * Results on my machine:
     * n=12 bench:  12,347,749 ns/iter (+/- 434,208)
     * n=13 bench:  25,198,094 ns/iter (+/- 487,870)
     * n=14 bench:  51,083,006 ns/iter (+/- 1,295,275)
     * n=15 bench: 102,719,961 ns/iter (+/- 1,512,884) <--
     * n=16 bench: 209,729,930 ns/iter (+/- 75,439,669)
     * n=17 bench: 425,023,594 ns/iter (+/- 143,358,926)
     * n=18 bench: 847,250,736 ns/iter (+/- 230,782,327)
     * n=19 bench: 1,774,238,775 ns/iter (+/- 430,690,312)
     * n=20 bench: 3,584,148,475 ns/iter (+/- 467,359,726)
     */
    ScryptParams::new(15, 8, 1)
}