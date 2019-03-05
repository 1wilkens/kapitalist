/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Result | Description |
/// | :--: | -- | -- | -- | -- |
/// | GET | `/transactions` | `from, to` | get transaction history |
/// | POST | `/transaction` | `TransactionCreationRequest` | create new transaction |
/// | GET | `/transaction/{tid}` | - | get transaction details |
/// | PUT | `/transaction/{tid}` | `TransactionUpdateRequest` | update transaction details |
/// | DELETE | `/transaction/{tid}` | - | delete transaction |
///
use actix_web::{http, AsyncResponder, Either, HttpResponse, Json, Path, Responder, State};
use futures::future::Future;
use slog::debug;

use kapitalist_types::request::{TransactionCreationRequest, TransactionUpdateRequest};

use crate::auth::UserGuard;
use crate::db::transaction::{
    CreateNewTransaction, DeleteTransaction, GetTransaction, GetTransactionsFromWallet, UpdateTransaction,
};
use crate::state::AppState;

pub fn get_all((state, user, wid): (State<AppState>, UserGuard, Path<i64>)) -> impl Responder {
    let get_txs = GetTransactionsFromWallet::new(user.user_id, *wid);
    state
        .db
        .send(get_txs)
        .and_then(move |res| {
            let resp = match res {
                Ok(txs) => HttpResponse::Ok().json(txs),
                Err(err) => {
                    debug!(&state.log, "Error getting transactions from database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn post((state, user, req): (State<AppState>, UserGuard, Json<TransactionCreationRequest>)) -> impl Responder {
    let new_tx = CreateNewTransaction::from_request(user.user_id, req.0);
    state
        .db
        .send(new_tx)
        .and_then(move |res| {
            let resp = match res {
                Ok(Some(tx)) => HttpResponse::Created()
                    .header(http::header::LOCATION, format!("/transaction/{}", tx.id))
                    .json(tx),
                Ok(None) => super::util::not_found(&"transaction"),
                Err(err) => {
                    debug!(&state.log, "Error inserting transaction into database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn get((state, user, tid): (State<AppState>, UserGuard, Path<i64>)) -> impl Responder {
    let get_tx = GetTransaction::new(user.user_id, *tid);
    state
        .db
        .send(get_tx)
        .and_then(move |res| {
            let resp = match res {
                Ok(Some(tx)) => HttpResponse::Ok().json(tx),
                Ok(None) => super::util::not_found(&"category"),
                Err(err) => {
                    debug!(&state.log, "Error getting transaction from database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn put(
    (state, user, tid, req): (State<AppState>, UserGuard, Path<i64>, Json<TransactionUpdateRequest>),
) -> impl Responder {
    if !req.is_valid() {
        return Either::A(super::util::update_request_invalid());
    }

    let update_tx = UpdateTransaction::from_request(user.user_id, *tid, req.0);
    Either::B(
        state
            .db
            .send(update_tx)
            .and_then(move |res| {
                let resp = match res {
                    Ok(Some(tx)) => HttpResponse::Ok().json(tx),
                    Ok(None) => super::util::not_found(&"transaction"),
                    Err(err) => {
                        debug!(&state.log, "Error updating transaction in database";
                        "error" => %&err);
                        super::util::internal_server_error()
                    }
                };
                Ok(resp)
            })
            .responder(),
    )
}

pub fn delete((state, user, tid): (State<AppState>, UserGuard, Path<i64>)) -> impl Responder {
    let delete_tx = DeleteTransaction::new(*tid, user.user_id);
    state
        .db
        .send(delete_tx)
        .and_then(move |res| {
            let resp = match res {
                Ok(true) => HttpResponse::Ok().json(""),
                Ok(false) => HttpResponse::NotFound().json("not found"),
                Err(err) => {
                    debug!(&state.log, "Error deleting transaction from database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}
