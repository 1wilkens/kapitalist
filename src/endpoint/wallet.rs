/* from doc/api.md
 *
 * ### Wallets / transactions
 * | Method | Endpoint | Payload/Params | Description |
 * | :--: | -- | -- | -- |
 * | POST | `/wallet` | WalletCreationRequest | create new wallet |
 * | GET | `/wallet/{wid}` | `id` | get wallet details |
 * | PUT | `/wallet/{wid}` | WalletUpdateRequest | update wallet details |
 * |
 * | GET | `/wallet/{wid}/transactions` | `from, to` | get transaction history |
 * | POST | `/wallet/{wid}/transaction` | TransactionCreationRequest | create new transaction |
 * | GET | `/wallet/{wid}/transaction/{tid}` | -- | get transaction details |
 * | PUT | `/wallet/{wid}/transaction/{tid}` | TransactionUpdateRequest | update transaction details |
 */

#[post("/")]
pub fn post() -> &'static str {
    "POST /wallet"
}

#[get("/<wid>")]
pub fn get(wid: u64) -> String {
    format!("GET /wallet/{}", wid)
}

#[put("/<wid>")]
pub fn put(wid: u64) -> String {
    format!("PUT /wallet/{}", wid)
}

#[get("/<wid>/transactions")]
pub fn tx_get_all(wid: u64) -> String {
    format!("GET /wallet/{}/transactions", wid)
}

#[post("/<wid>/transaction")]
pub fn tx_post(wid: u64) -> String {
    format!("POST /wallet/{}/transaction", wid)
}

#[get("/<wid>/transaction/<tid>")]
pub fn tx_get(wid: u64, tid: u64) -> String {
    format!("POST /wallet/{}/transaction/{}", wid, tid)
}

#[put("/<wid>/transaction/<tid>")]
pub fn tx_put(wid: u64, tid: u64) -> String {
    format!("PUT /wallet/{}/transaction/{}", wid, tid)
}