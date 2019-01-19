use actix_web::HttpResponse;

use crate::response::ErrorResponse;

pub fn not_found(entity: &str) -> HttpResponse {
    return HttpResponse::NotFound().json(ErrorResponse::new(format!("{} not found", entity)));
}

pub fn unauthorized() -> HttpResponse {
    return HttpResponse::Unauthorized().json(ErrorResponse::unauthorized());
}
