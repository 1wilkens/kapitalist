use actix_web::{AsyncResponder, HttpResponse, Json, Responder, State};
use futures::Future;
use slog::{debug, trace};

use crate::auth::UserGuard;
use crate::db::category::{GetCategory, NewCategory};
use crate::request::CategoryCreationRequest;
use crate::response::ErrorResponse;
use crate::state::AppState;

pub fn post((state, user, req): (State<AppState>, UserGuard, Json<CategoryCreationRequest>)) -> impl Responder {
    trace!(&state.log, "Endpoint {ep} called", ep = "category::post"; "request" => ?&req.0);

    let new_category = NewCategory::from_request(req.0, user.user_id);
    state
        .db
        .send(new_category)
        .and_then(move |res| {
            let resp = match res {
                Ok(category) => HttpResponse::Ok().json(category),
                Err(err) => {
                    debug!(&state.log, "Error inserting category into database";
                        "error" => %&err);
                    HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error())
                }
            };

            trace!(&state.log, "Endpoint {ep} returned", ep = "wallet::post";
                            "response" => ?&resp.body(),
                            "statuscode" => %&resp.status());
            Ok(resp)
        })
        .responder()
}

pub fn get((_state, _user): (State<AppState>, UserGuard)) -> impl Responder {
    use crate::db::schema::categories;

    /*let c = categories::table
    .filter(categories::columns::id.eq(id))
    .get_result(&*db)
    .map_err(|_| Json(ErrorResponse::server_error()))?;*/

    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}
