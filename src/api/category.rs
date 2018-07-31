/*use actix_web::Json;

use auth::UserGuard;
use db::DbConn;

use model::{Category};
use request::*;
use response::*;

//#[get("/<id>")]
pub fn get(db: DbConn, _user: UserGuard, id: i32) -> Result<Json<Category>, Json<ErrorResponse>> {
    use schema::categories;

    let c = categories::table
        .filter(categories::columns::id.eq(id))
        .get_result(&*db)
        .map_err(|_| Json(ErrorResponse::server_error()))?;

    Ok(Json(c))
}*/
