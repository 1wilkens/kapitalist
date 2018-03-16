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

#[post("/register")]
pub fn register() -> &'static str {
    "POST /register"
}

#[get("/me")]
pub fn get_me() -> &'static str {
    "GET /me"
}

#[put("/me")]
pub fn put_me() -> &'static str {
    "PUT /me"
}

#[post("/authenticate")]
pub fn authenticate() -> &'static str {
    "POST /authenticate"
}