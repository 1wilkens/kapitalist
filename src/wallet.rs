/* from doc/api.md
 *
 * ### wallets
 * - POST /wallet -- (create new wallet)
 * - GET /wallet/{id} -- (get wallet details)
 * - PUT /wallet/{id} -- (update wallet details)
 */

#[post("/")]
pub fn post() -> &'static str {
    "POST /wallet"
}

#[get("/<id>")]
pub fn get(id: u64) -> String {
    format!("GET /wallet/{}", id)
}

#[put("/<id>")]
pub fn put(id: i64) -> String {
    format!("PUT /wallet/{}", id)
}