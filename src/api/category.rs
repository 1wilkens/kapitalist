use actix_web::{AsyncResponder, HttpResponse, Json, Responder, State};

use crate::auth::UserGuard;
use crate::db::category::{GetCategory, NewCategory};
use crate::request::CategoryCreationRequest;
use crate::response::ErrorResponse;
use crate::state::AppState;

pub fn get((_state, _user): (State<AppState>, UserGuard)) -> impl Responder {
    use crate::db::schema::categories;

    /*let c = categories::table
    .filter(categories::columns::id.eq(id))
    .get_result(&*db)
    .map_err(|_| Json(ErrorResponse::server_error()))?;*/

    HttpResponse::InternalServerError().json(ErrorResponse::not_implemented())
}
