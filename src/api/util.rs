use serde::Serialize;
use warp::{
    http::{header, StatusCode},
    reject, reply, Rejection, Reply,
};

use crate::err::Error;

pub(crate) fn created<T: Serialize>(resp: &T, url: String) -> impl Reply {
    reply::with_status(
        reply::with_header(reply::json(resp), header::LOCATION, url),
        StatusCode::CREATED,
    )
}

pub(crate) fn bad_request<T: Into<String>>(msg: T) -> Rejection {
    error(Error::BadRequest(msg.into()))
}

pub(crate) fn not_found<T: Into<String>>(entity: T) -> Rejection {
    error(Error::NotFound(entity.into()))
}

pub(crate) fn error<T: Into<Error>>(error: T) -> Rejection {
    reject::custom(error.into())
}
