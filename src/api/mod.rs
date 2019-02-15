use actix_web::{HttpRequest, Json};
use kapitalist_types::response::VersionResponse;

use crate::state::AppState;

pub mod category;
pub mod transaction;
pub mod user;
pub mod wallet;

pub mod util;

pub fn index(_: &HttpRequest<AppState>) -> String {
    "Kapitalist is running".into()
}

pub fn version(_: &HttpRequest<AppState>) -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
