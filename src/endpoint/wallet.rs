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

use rocket_contrib::Json;
use rocket::response::status;

use auth::UserGuard;
use db::DbConn;
use model::{Wallet, NewWallet};
use request::WalletCreationRequest;
use response::ErrorResponse;

#[post("/", data = "<req>")]
pub fn post(_db: DbConn, _user: UserGuard, req: Json<WalletCreationRequest>) -> Result<status::Created<Json<Wallet>>, Json<ErrorResponse>> {
    println!("POST /wallet: name={}, balance={}, color={}", &req.name, &req.balance, &req.color);
    let _new_wallet = NewWallet::from_request(req.0);
    Err(Json(ErrorResponse::not_implemented()))
}

#[get("/<wid>")]
pub fn get(_db: DbConn, _user: UserGuard, wid: u64) -> Result<Json<Wallet>, Json<ErrorResponse>> {
    println!("GET /wallet/{}", wid);
    Err(Json(ErrorResponse::not_implemented()))
}

#[put("/<wid>")]
pub fn put(_db: DbConn, _user: UserGuard, wid: u64) -> Result<Json<Wallet>, Json<ErrorResponse>> {
    println!("PUT /wallet/{}", wid);
    Err(Json(ErrorResponse::not_implemented()))
}

#[get("/<wid>/transactions")]
pub fn tx_get_all(_db: DbConn, _user: UserGuard, wid: u64) -> Result<Json<()>, Json<ErrorResponse>> {
    println!("GET /wallet/{}/transactions", wid);
    Err(Json(ErrorResponse::not_implemented()))
}

#[post("/<wid>/transaction")]
pub fn tx_post(_db: DbConn, _user: UserGuard, wid: u64) -> Result<Json<()>, Json<ErrorResponse>> {
    println!("POST /wallet/{}/transaction", wid);
    Err(Json(ErrorResponse::not_implemented()))
}

#[get("/<wid>/transaction/<tid>")]
pub fn tx_get(_db: DbConn, _user: UserGuard, wid: u64, tid: u64) -> Result<Json<()>, Json<ErrorResponse>> {
    println!("POST /wallet/{}/transaction/{}", wid, tid);
    Err(Json(ErrorResponse::not_implemented()))
}

#[put("/<wid>/transaction/<tid>")]
pub fn tx_put(_db: DbConn, _user: UserGuard, wid: u64, tid: u64) -> Result<Json<()>, Json<ErrorResponse>> {
    println!("PUT /wallet/{}/transaction/{}", wid, tid);
    Err(Json(ErrorResponse::not_implemented()))
}