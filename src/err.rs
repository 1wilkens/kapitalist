use std::convert::Infallible;

use warp::{Rejection, Reply};

pub async fn handle_rejection(_: Rejection) -> Result<impl Reply, Infallible> {
    // FIXME: proper error handling
    Ok("FIXME".to_string())
}
