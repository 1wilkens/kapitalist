use kapitalist_types::response::{ErrorResponse, VersionResponse};
use rocket::response::{content, status};
use rocket_contrib::json::Json;

//pub mod category;
//pub mod transaction;
pub mod user;
//pub mod wallet;

pub mod util;

type Result<S> = std::result::Result<S, status::Custom<Json<ErrorResponse>>>;

#[get("/")]
pub fn index() -> content::Plain<String> {
    content::Plain("Kapitalist is running".into())
}

#[get("/version")]
pub fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
