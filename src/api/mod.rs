use crate::state::AppState;

use actix_web::HttpRequest;

pub mod category;
pub mod transaction;
pub mod user;
pub mod wallet;

pub mod util;

pub fn index(_req: &HttpRequest<AppState>) -> String {
    "Kapitalist is running".into()
}
