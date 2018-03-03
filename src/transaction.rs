/* from doc/api.md
 *
 * ### transactions
 * - POST /transaction -- (create new transaction)
 * - GET /transaction/{id} -- (get transaction details)
 * - PUT /transaction/{id} -- (update transaction details)
 */

#[post("/")]
pub fn post() -> &'static str {
    "POST /transaction"
}

#[get("/<id>")]
pub fn get(id: u64) -> String {
    format!("GET /transaction/{}", id)
}

#[put("/<id>")]
pub fn put(id: i64) -> String {
    format!("PUT /transaction/{}", id)
}