use state::AppState;

use actix_web::HttpRequest;

//pub mod category;
pub mod user;
pub mod wallet;

pub fn index(_req: &HttpRequest<AppState>) -> String {
    "Kapitalist is running".into()
}
