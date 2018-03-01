/* from doc/api.md
 *
 * ### wallets
 * - POST /wallets -- (create new wallet)
 * - GET /wallets -- (get own wallets)
 * - GET /wallets/{id} -- (get wallet details)
 * - PUT /wallets/{id} -- (update wallet details)
 */

#[get("/")]
pub fn get() -> &'static str {
    "GET /wallets"
}

#[get("/<id>")]
pub fn get_one(id: u64) -> String {
    format!("GET /wallets/{}", id)
}

#[post("/")]
pub fn post() -> &'static str {
    "POST /wallets"
}

#[put("/<id>")]
pub fn put(id: i64) -> String {
    format!("PUT /wallets/{}", id)
}