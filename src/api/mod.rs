use std::convert::Infallible;

use tracing::trace;
use warp::{reply, Reply};

use kapitalist_types::response::VersionResponse;

pub mod category;
pub mod transaction;
pub mod user;
pub mod wallet;

pub mod util;

pub async fn index() -> Result<impl Reply, Infallible> {
    trace!("index()");
    Ok(format!(
        "{} v{} is running",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ))
}

pub async fn version() -> Result<impl Reply, Infallible> {
    trace!("version()");
    Ok(reply::json(&VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}
