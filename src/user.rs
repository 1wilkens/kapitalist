/* from doc/api.md
 *
 * ### user management / authentication
 * - GET /me -- (get own user details)
 * - PUT /me -- (update own user details)
 * 
 * - POST /user  -- (register new user)
 * - POST /auth  -- (obtain authentication token)
 */

#[post("/me")]
pub fn post() -> &'static str {
    "POST /user"
}

#[get("/me")]
pub fn get() -> &'static str {
    "GET /me"
}

#[put("/me")]
pub fn put() -> &'static str {
    "PUT /me"
}

#[post("/auth")]
pub fn token() -> &'static str {
    "POST /auth"
}