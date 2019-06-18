/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Description |
/// | :--: | -- | -- | -- |
/// | POST | `/wallet` | `WalletCreationRequest` | create new wallet |
/// | GET | `/wallet/{wid}` | `id` | get wallet details |
/// | PUT | `/wallet/{wid}` | `WalletUpdateRequest` | update wallet details |
/// | DELETE | `/wallet/{wid}` | -- | delete wallet |
///
use actix_web::{http, AsyncResponder, Either, HttpResponse, Json, Path, Responder, State};
use futures::Future;
use slog::debug;

use kapitalist_types::request::{WalletCreationRequest, WalletUpdateRequest};

use crate::auth::UserGuard;
use crate::db::wallet::{DeleteWallet, GetWallet, GetWalletsFromUser, NewWallet, UpdateWallet};
use crate::state::AppState;

pub fn get_all((state, user): (State<AppState>, UserGuard)) -> impl Responder {
    let get_wallets = GetWalletsFromUser::new(user.user_id);
    state
        .db
        .send(get_wallets)
        .and_then(move |res| {
            let resp = match res {
                Ok(wallets) => HttpResponse::Ok().json(wallets),
                Err(err) => {
                    debug!(&state.log, "Error getting wallets from database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn post((state, user, req): (State<AppState>, UserGuard, Json<WalletCreationRequest>)) -> impl Responder {
    let new_wallet = NewWallet::from_request(user.user_id, req.0);
    state
        .db
        .send(new_wallet)
        .and_then(move |res| {
            let resp = match res {
                // XXX: Set location header
                Ok(wallet) => HttpResponse::Created()
                    .header(http::header::LOCATION, format!("/wallet/{}", wallet.id))
                    .json(wallet.into_response()),
                Err(err) => {
                    debug!(&state.log, "Error inserting wallet into database";
                        "error" => %&err);
                    super::util::internal_server_error()
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
                Ok(Some(wallet)) => HttpResponse::Ok().json(wallet.into_response()),
                Ok(None) => super::util::not_found(&"wallet"),
                Err(err) => {
                    debug!(&state.log, "Error getting wallet from database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn put(
    (state, user, wid, req): (State<AppState>, UserGuard, Path<i64>, Json<WalletUpdateRequest>),
) -> impl Responder {
    if !req.is_valid() {
        // At least one field has to be set, could also return 301 unchanged?
        return Either::A(super::util::update_request_invalid());
    }

    let update_wallet = UpdateWallet::from_request(user.user_id, *wid, req.0);
    Either::B(
        state
            .db
            .send(update_wallet)
            .and_then(move |res| {
                let resp = match res {
                    Ok(Some(wallet)) => HttpResponse::Ok().json(wallet.into_response()),
                    Ok(None) => super::util::not_found(&"wallet"),
                    Err(err) => {
                        debug!(&state.log, "Error updating wallet in database";
                        "error" => %&err);
                        super::util::internal_server_error()
                    }
                };
                Ok(resp)
            })
            .responder(),
    )
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
                    debug!(&state.log, "Error deleting wallet from database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}
