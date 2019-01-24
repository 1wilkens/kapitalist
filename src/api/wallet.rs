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
use slog::debug;

use crate::auth::UserGuard;
use crate::db::wallet::{DeleteWallet, GetWallet, NewWallet, UpdateWallet};
use crate::request::{WalletCreationRequest, WalletUpdateRequest};
use crate::response::ErrorResponse;
use crate::state::AppState;

pub fn post((state, user, req): (State<AppState>, UserGuard, Json<WalletCreationRequest>)) -> impl Responder {
    let new_wallet = NewWallet::from_request(user.user_id, req.0);
    state
        .db
        .send(new_wallet)
        .and_then(move |res| {
            let resp = match res {
                // XXX: Set location header
                Ok(wallet) => HttpResponse::Created().json(wallet),
                Err(err) => {
                    debug!(&state.log, "Error inserting wallet into database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn get((state, user, wid): (State<AppState>, UserGuard, Path<i64>)) -> impl Responder {
    let get_wallet = GetWallet::new(user.user_id, *wid);
    state
        .db
        .send(get_wallet)
        .and_then(move |res| {
            let resp = match res {
                Ok(Some(wallet)) => HttpResponse::Ok().json(wallet),
                // XXX: Handle this properly and add utility method for 404
                Ok(None) => super::util::not_found(&"wallet"),
                Err(err) => {
                    debug!(&state.log, "Error getting wallet from database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn put(
    (state, user, wid, req): (State<AppState>, UserGuard, Path<i64>, Json<WalletUpdateRequest>),
) -> impl Responder {
    let update_wallet = UpdateWallet::from_request(user.user_id, *wid, req.0);
    state
        .db
        .send(update_wallet)
        .and_then(move |res| {
            let resp = match res {
                Ok(Some(wallet)) => HttpResponse::Ok().json(wallet),
                // XXX: Handle this properly and add utility method for 404
                Ok(None) => super::util::not_found(&"wallet"),
                Err(err) => {
                    debug!(&state.log, "Error updating wallet in database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn delete((state, user, wid): (State<AppState>, UserGuard, Path<i64>)) -> impl Responder {
    let delete_wallet = DeleteWallet::new(user.user_id, *wid);
    state
        .db
        .send(delete_wallet)
        .and_then(move |res| {
            let resp = match res {
                Ok(true) => HttpResponse::Ok().json(""),
                Ok(false) => super::util::not_found(&"wallet"),
                Err(err) => {
                    debug!(&state.log, "Error delete wallet from database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}
