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

use actix_web::{AsyncResponder, HttpResponse, Json, Path, Responder, State};
use futures::Future;

use crate::auth::UserGuard;
use crate::db::wallet::{GetWallet, NewWallet};
use crate::request::WalletCreationRequest;
use crate::response::ErrorResponse;
use crate::state::AppState;

pub fn post((state, user, req): (State<AppState>, UserGuard, Json<WalletCreationRequest>)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::post"; "request" => ?&req.0);

    let new_wallet = NewWallet::from_request(req.0, user.user_id);
    state
        .db
        .send(new_wallet)
        .and_then(move |res| {
            let resp = match res {
                Ok(wallet) => HttpResponse::Ok().json(wallet),
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

pub fn get((state, user, wid): (State<AppState>, UserGuard, Path<(i32)>)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::get");
    let get_wallet = GetWallet::new(*wid, user.user_id);
    state
        .db
        .send(get_wallet)
        .and_then(move |res| {
            let resp = match res {
                Ok(Some(wallet)) => HttpResponse::Ok().json(wallet),
                // XXX: Handle this properly and add utility method for 404
                Ok(None) => HttpResponse::NotFound().json("not found"),
                Err(err) => {
                    debug!(&state.log, "Error getting wallet from database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };

            trace!(&state.log, "Endpoint {ep} returned", ep = "wallet::get";
                            "response" => ?&resp.body(),
                            "statuscode" => %&resp.status());
            Ok(resp)
        }).responder()
}

pub fn put((state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "wallet::put");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

