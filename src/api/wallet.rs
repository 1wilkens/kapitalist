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

use actix_web::{AsyncResponder, HttpResponse, Json, Responder, State};
use futures::Future;

use auth::UserGuard;
use db::wallet::{NewWallet, Wallet};
use request::WalletCreationRequest;
use response::ErrorResponse;
use state::AppState;

pub fn post(
    (state, _user, req): (State<AppState>, UserGuard, Json<WalletCreationRequest>),
) -> impl Responder {
    eprintln!(
        "POST /wallet: name={}, balance={}, color={}",
        &req.name, &req.balance, &req.color
    );
    let new_wallet = NewWallet::from_request(req.0);
    state
        .db
        .send(new_wallet)
        .and_then(|res| match res {
            Ok(user) => {
                let resp = HttpResponse::Ok().json(user);
                eprintln!("{:?}", resp);
                Ok(resp)
            }
            Err(e) => {
                eprintln!("{:?}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn get((_state, _user, wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    eprintln!("GET /wallet/{}", wid);
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn put((_state, _user, wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    eprintln!("PUT /wallet/{}", wid);
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn tx_get_all((_state, _user, wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    eprintln!("GET /wallet/{}/transactions", wid);
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn tx_post((_state, _user, wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    eprintln!("POST /wallet/{}/transaction", wid);
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn tx_get((_state, _user, wid, tid): (State<AppState>, UserGuard, u64, u64)) -> impl Responder {
    eprintln!("POST /wallet/{}/transaction/{}", wid, tid);
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn tx_put((_state, _user, wid, tid): (State<AppState>, UserGuard, u64, u64)) -> impl Responder {
    eprintln!("PUT /wallet/{}/transaction/{}", wid, tid);
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}
