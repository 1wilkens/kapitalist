use actix_web::HttpResponse;

use kapitalist_types::response::ErrorResponse;

pub fn internal_server_error() -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
}

pub fn not_found(entity: &str) -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResponse::new(format!("{} not found", entity)))
}

pub fn unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().json(ErrorResponse::unauthorized())
}

pub fn update_request_invalid() -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorResponse::new(
        "Request has to contain at least one field to update",
    ))
}
