use actix_web::{AsyncResponder, HttpResponse, Json, Responder, State};
use slog::trace;

use crate::auth::UserGuard;
/*use crate::db::category::{GetCategory, NewCategory};
use crate::request::CategoryCreationRequest;*/
use crate::response::ErrorResponse;
use crate::state::AppState;

pub fn get_all((_state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn post((_state, _user, _wid): (State<AppState>, UserGuard, u64)) -> impl Responder {
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn get((_state, _user, _wid, _tid): (State<AppState>, UserGuard, u64, u64)) -> impl Responder {
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}

pub fn put((_state, _user, _wid, _tid): (State<AppState>, UserGuard, u64, u64)) -> impl Responder {
    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}
