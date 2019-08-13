/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Result | Description |
/// | :--: | -- | -- | -- | -- |
/// | GET | `/transactions` | `from, to` | get transaction history |
/// | POST | `/transaction` | `TransactionCreationRequest` | create new transaction |
/// | GET | `/transaction/{tid}` | - | get transaction details |
/// | PUT | `/transaction/{tid}` | `TransactionUpdateRequest` | update transaction details |
/// | DELETE | `/transaction/{tid}` | - | delete transaction |
///
use rocket::{response::status, State};
use rocket_contrib::json::Json;
use slog::debug;

use kapitalist_types::request::{CategoryCreationRequest, CategoryUpdateRequest};
use kapitalist_types::response::CategoryResponse;

use crate::api::util::{internal_server_error, not_found, update_request_invalid};
use crate::auth::User;
use crate::db::{
    category::{Category, DeleteCategory, GetCategoriesForUser, GetCategory, NewCategory, UpdateCategory},
    Database,
};
use crate::state::AppState;

#[post("/", data = "<req>")]
pub fn post(
    user: User,
    state: State<AppState>,
    db: Database,
    req: Json<CategoryCreationRequest>,
) -> super::Result<status::Created<Json<CategoryResponse>>> {
    let new_category = NewCategory::from_request(req.0, user.user_id);
    match new_category.execute(&*db) {
        Ok(category) => {
            let url = format!("/category/{}", category.id);
            Ok(status::Created(url, Some(Json(category.into_response()))))
        }
        Err(err) => {
            debug!(&state.log, "Error inserting category into database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[get("/all")]
pub fn get_all(user: User, state: State<AppState>, db: Database) -> super::Result<Json<Vec<CategoryResponse>>> {
    let get_categories = GetCategoriesForUser::new(user.user_id);
    match get_categories.execute(&*db) {
        Ok(txs) => Ok(Json(txs.into_iter().map(Category::into_response).collect())),
        Err(err) => {
            debug!(&state.log, "Error getting categories from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[get("/<cid>")]
pub fn get(user: User, state: State<AppState>, db: Database, cid: i64) -> super::Result<Json<CategoryResponse>> {
    let get_category = GetCategory::new(cid, user.user_id);
    match get_category.execute(&*db) {
        Ok(Some(category)) => Ok(Json(category.into_response())),
        Ok(None) => Err(not_found("category")),
        Err(err) => {
            debug!(&state.log, "Error getting category from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[put("/<cid>", data = "<req>")]
pub fn put(
    user: User,
    state: State<AppState>,
    db: Database,
    cid: i64,
    req: Json<CategoryUpdateRequest>,
) -> super::Result<Json<CategoryResponse>> {
    if !req.is_valid() {
        // At least one field has to be set, could also return 301 unchanged?
        return Err(update_request_invalid());
    }

    let update_category = UpdateCategory::from_request(user.user_id, cid, req.0);
    match update_category.execute(&*db) {
        Ok(Some(category)) => Ok(Json(category.into_response())),
        Ok(None) => Err(not_found("category")),
        Err(err) => {
            debug!(&state.log, "Error getting category from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[delete("/<cid>")]
pub fn delete(user: User, state: State<AppState>, db: Database, cid: i64) -> super::Result<Json<()>> {
    let delete_category = DeleteCategory::new(user.user_id, cid);
    match delete_category.execute(&*db) {
        Ok(true) => Ok(Json(())),
        Ok(false) => Err(not_found("category")),
        Err(err) => {
            debug!(&state.log, "Error getting category from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}
