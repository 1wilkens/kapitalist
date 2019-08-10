use rocket_contrib::json::Json;
use kapitalist_types::response::VersionResponse;

use crate::state::AppState;

//pub mod category;
//pub mod transaction;
//pub mod user;
//pub mod wallet;

//pub mod util;

#[get("/")]
pub fn index() -> String {
    "Kapitalist is running".into()
}

pub fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
