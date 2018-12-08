use actix_web::{AsyncResponder, HttpResponse, Json, Responder, State};
use slog::trace;

use crate::auth::UserGuard;
/*use crate::db::category::{GetCategory, NewCategory};
use crate::request::CategoryCreationRequest;*/
use crate::response::ErrorResponse;
use crate::state::AppState;

pub fn get_all((state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "transaction::get_all");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn post((state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "transaction::post");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn get((state, _user, _wid, _tid): (State<AppState>, UserGuard, u64, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "transaction::get");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn put((state, _user, _wid, _tid): (State<AppState>, UserGuard, u64, u64)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "transaction::put");
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}
