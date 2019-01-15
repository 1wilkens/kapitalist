use actix_web::{AsyncResponder, HttpResponse, Json, Path, Responder, State};
use futures::future::Future;
use slog::debug;

use crate::auth::UserGuard;
use crate::db::transaction::{DeleteTransaction, GetTransaction, GetTransactionsFromWallet, NewTransaction};
use crate::request::TransactionCreationRequest;
use crate::response::ErrorResponse;
use crate::state::AppState;

pub fn get_all((state, user, wid): (State<AppState>, UserGuard, Path<i32>)) -> impl Responder {
    let get_txs = GetTransactionsFromWallet::new(*wid, user.user_id);
    state
        .db
        .send(get_txs)
        .and_then(move |res| {
            let resp = match res {
                Ok(txs) => HttpResponse::Ok().json(txs),
                Err(err) => {
                    debug!(&state.log, "Error getting transactions from database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn post((state, user, req): (State<AppState>, UserGuard, Json<TransactionCreationRequest>)) -> impl Responder {
    // XXX: This currently does NOT check if the user owns the source wallet
    // Unfortunately we can't just add a user_id field to NewTransaction as it is directly
    // Insertable. TODO: Figure out an elegant way to handle this!
    let new_tx = NewTransaction::from_request(req.0);
    state
        .db
        .send(new_tx)
        .and_then(move |res| {
            let resp = match res {
                Ok(tx) => HttpResponse::Ok().json(tx),
                Err(err) => {
                    debug!(&state.log, "Error inserting transaction into database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn get((state, user, tid): (State<AppState>, UserGuard, Path<i32>)) -> impl Responder {
    let get_tx = GetTransaction::new(*tid, user.user_id);
    state
        .db
        .send(get_tx)
        .and_then(move |res| {
            let resp = match res {
                Ok(tx) => HttpResponse::Ok().json(tx),
                Err(err) => {
                    debug!(&state.log, "Error getting transaction from database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn put(
    (state, user, tid /*, req*/): (
        State<AppState>,
        UserGuard,
        Path<i32>, /*, Json<TransactionUpdateRequest>*/
    ),
) -> impl Responder {
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn delete((state, user, tid): (State<AppState>, UserGuard, Path<i32>)) -> impl Responder {
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
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}
