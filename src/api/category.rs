use actix_web::{AsyncResponder, HttpResponse, Json, Path, Responder, State};
use futures::Future;
use slog::{debug, trace};

use crate::auth::UserGuard;
use crate::db::category::{GetCategory, NewCategory};
use crate::request::CategoryCreationRequest;
use crate::response::ErrorResponse;
use crate::state::AppState;

pub fn post((state, user, req): (State<AppState>, UserGuard, Json<CategoryCreationRequest>)) -> impl Responder {
    let new_category = NewCategory::from_request(req.0, user.user_id);
    state
        .db
        .send(new_category)
        .and_then(move |res| {
            let resp = match res {
                Ok(category) => HttpResponse::Ok().json(category),
                Err(err) => {
                    debug!(&state.log, "Error inserting category into database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}

pub fn get((state, user, tid): (State<AppState>, UserGuard, Path<i32>)) -> impl Responder {
    let get_category = GetCategory::new(*tid, user.user_id);
    state
        .db
        .send(get_category)
        .and_then(move |res| {
            let resp = match res {
                Ok(Some(category)) => HttpResponse::Ok().json(category),
                // XXX: Handle this properly and add utility method for 404
                Ok(None) => HttpResponse::NotFound().json("not found"),
                Err(err) => {
                    debug!(&state.log, "Error getting category from database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };
            Ok(resp)
        })
        .responder()
}
