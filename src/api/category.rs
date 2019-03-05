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
use futures::Future;
use slog::debug;

use kapitalist_types::request::{CategoryCreationRequest, CategoryUpdateRequest};

use crate::auth::UserGuard;
use crate::db::category::{DeleteCategory, GetCategory, NewCategory, UpdateCategory};
use crate::state::AppState;

pub fn post((state, user, req): (State<AppState>, UserGuard, Json<CategoryCreationRequest>)) -> impl Responder {
    let new_category = NewCategory::from_request(req.0, user.user_id);
    state
        .db
        .send(new_category)
        .and_then(move |res| {
            let resp = match res {
                Ok(category) => HttpResponse::Created()
                    .header(http::header::LOCATION, format!("/category/{}", category.id))
                    .json(category),
                Err(err) => {
                    debug!(&state.log, "Error inserting category into database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn get((state, user, tid): (State<AppState>, UserGuard, Path<i64>)) -> impl Responder {
    let get_category = GetCategory::new(*tid, user.user_id);
    state
        .db
        .send(get_category)
        .and_then(move |res| {
            let resp = match res {
                Ok(Some(category)) => HttpResponse::Ok().json(category),
                Ok(None) => super::util::not_found(&"category"),
                Err(err) => {
                    debug!(&state.log, "Error getting category from database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn put(
    (state, user, tid, req): (State<AppState>, UserGuard, Path<i64>, Json<CategoryUpdateRequest>),
) -> impl Responder {
    if !req.is_valid() {
        // At least one field has to be set, could also return 301 unchanged?
        return Either::A(super::util::update_request_invalid());
    }

    let update_category = UpdateCategory::from_request(user.user_id, *tid, req.0);
    Either::B(
        state
            .db
            .send(update_category)
            .and_then(move |res| {
                let resp = match res {
                    Ok(Some(category)) => HttpResponse::Ok().json(category),
                    Ok(None) => super::util::not_found(&"category"),
                    Err(err) => {
                        debug!(&state.log, "Error getting category from database";
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
    let delete_category = DeleteCategory::new(user.user_id, *tid);
    state
        .db
        .send(delete_category)
        .and_then(move |res| {
            let resp = match res {
                Ok(true) => HttpResponse::Ok().json(""),
                Ok(false) => super::util::not_found(&"category"),
                Err(err) => {
                    debug!(&state.log, "Error getting category from database";
                        "error" => %&err);
                    super::util::internal_server_error()
                }
            };
            Ok(resp)
        })
        .responder()
}
