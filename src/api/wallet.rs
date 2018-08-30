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
use db::wallet::NewWallet;
use request::WalletCreationRequest;
use response::ErrorResponse;
use state::AppState;

pub fn post((state, _user, req): (State<AppState>, UserGuard, Json<WalletCreationRequest>)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::post"; "request" => ?&req.0);

    let new_wallet = NewWallet::from_request(req.0);
    state
        .db
        .send(new_wallet)
        .and_then(move |res| {
            let resp = match res {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(err) => {
                    debug!(&state.log, "Error inserting wallet into database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };

            trace!(&state.log, "Endpoint {ep} returned", ep = "wallet::post";
                            "response" => ?&resp.body(),
                            "statuscode" => %&resp.status());
            Ok(resp)
        })
        .responder()
}

pub fn get((state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::get");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn put((state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::put");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn tx_get_all((state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::tx_get_all");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn tx_post((state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::tx_post");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn tx_get(
    (state, _user, _wid, _tid): (State<AppState>, UserGuard, u64, u64),
) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::tx_get");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn tx_put(
    (state, _user, _wid, _tid): (State<AppState>, UserGuard, u64, u64),
) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::tx_put");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}
