use std::convert::Infallible;

use warp::{http::StatusCode, Rejection, Reply};

use kapitalist_types::response::ErrorResponse;

#[derive(Debug)]
pub enum Error {
    DbError(diesel::result::Error),
    PoolError,
    BadRequest(String),
    NotFound(String),
    Unauthorized,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let mut code = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message = "Internal server error";
    let mut resp = None;

    if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    }

    if resp.is_none() {
        resp = Some(ErrorResponse::new(message));
    }

    let json = warp::reply::json(&resp.unwrap());
    Ok(warp::reply::with_status(json, code))
}

impl warp::reject::Reject for Error {}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Self::DbError(error)
    }
}
