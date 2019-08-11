use rocket::{
    http::Status,
    response::{status, Responder},
};
use rocket_contrib::json::Json;

use kapitalist_types::response::ErrorResponse;

// Status shorthands. These are private as they should not be used directly
fn status_bad_request<'r, R: Responder<'r>>(responder: R) -> status::Custom<R> {
    status::Custom(Status::BadRequest, responder)
}

fn status_unauthorized<'r, R: Responder<'r>>(responder: R) -> status::Custom<R> {
    status::Custom(Status::Unauthorized, responder)
}

fn status_not_found<'r, R: Responder<'r>>(responder: R) -> status::Custom<R> {
    status::Custom(Status::NotFound, responder)
}

fn status_internal_server_error<'r, R: Responder<'r>>(responder: R) -> status::Custom<R> {
    status::Custom(Status::InternalServerError, responder)
}

// Common request shorthands
pub fn internal_server_error() -> status::Custom<Json<ErrorResponse>> {
    status_internal_server_error(Json(ErrorResponse::internal_server_error()))
}

// Cannot use status::BadRequest here, as we need a single return type for handlers
pub fn bad_request<S: Into<String>>(msg: S) -> status::Custom<Json<ErrorResponse>> {
    status_bad_request(Json(ErrorResponse::new(msg.into())))
}

pub fn update_request_invalid() -> status::Custom<Json<ErrorResponse>> {
    bad_request("Request has to contain at least one field to update")
}

pub fn unauthorized() -> status::Custom<Json<ErrorResponse>> {
    status_unauthorized(Json(ErrorResponse::unauthorized()))
}

pub fn not_found(entity: &str) -> status::Custom<Json<ErrorResponse>> {
    status_not_found(Json(ErrorResponse::new(format!("{} not found", entity))))
}
