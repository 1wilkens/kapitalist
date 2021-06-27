/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Description |
/// | :--: | -- | -- | -- |
/// | POST | `/category` | `CategoryCreationRequest` | create new category |
/// | GET | `/category/{cid}` | - | get category details |
/// | PUT | `/category/{cid}` | `CategoryUpdateRequest` | update category details |
/// | DELETE | `/category/{cid}` | - | delete category |
/// | GET | `/category/all` | `from, to` | get all available category for the user |
///
use tracing::debug;
use warp::{reject, reply, Rejection, Reply};

use kapitalist_types::request::{CategoryCreationRequest, CategoryUpdateRequest};
use kapitalist_types::response::CategoryResponse;

use crate::api::util;
use crate::auth::User;
use crate::db::{
    category::{
        Category, DeleteCategory, GetCategoriesForUser, GetCategory, NewCategory, UpdateCategory,
    },
    Database,
};

pub async fn post(
    db: Database,
    user: User,
    req: CategoryCreationRequest,
) -> Result<impl Reply, Rejection> {
    let new_category = NewCategory::from_request(req, user.user_id);
    match new_category.execute(&*db.0) {
        Ok(category) => {
            let url = format!("/category/{}", category.id);
            Ok(util::created(&category.into_response(), url))
        }
        Err(err) => {
            debug!(error = %&err, "Error inserting category into database");
            Err(util::error(err))
        }
    }
}

pub async fn get(db: Database, user: User, cid: i64) -> Result<impl Reply, Rejection> {
    let get_category = GetCategory::new(cid, user.user_id);
    match get_category.execute(&*db.0) {
        Ok(Some(category)) => Ok(reply::json(&category.into_response())),
        Ok(None) => Err(util::not_found("category")),
        Err(err) => {
            debug!(error = %&err, "Error getting category from database");
            Err(util::error(err))
        }
    }
}

pub async fn put(
    db: Database,
    user: User,
    cid: i64,
    req: CategoryUpdateRequest,
) -> Result<impl Reply, Rejection> {
    if !req.is_valid() {
        // At least one field has to be set, could also return 301 unchanged?
        return Err(reject::reject());
    }

    let update_category = UpdateCategory::from_request(user.user_id, cid, req);
    match update_category.execute(&*db.0) {
        Ok(Some(category)) => Ok(reply::json(&category.into_response())),
        Ok(None) => Err(util::not_found("category")),
        Err(err) => {
            debug!(error = %&err, "Error getting category from database");
            Err(util::error(err))
        }
    }
}

pub async fn delete(db: Database, user: User, cid: i64) -> Result<impl Reply, Rejection> {
    let delete_category = DeleteCategory::new(user.user_id, cid);
    match delete_category.execute(&*db.0) {
        Ok(true) => Ok(reply::json(&())),
        Ok(false) => Err(util::not_found("category")),
        Err(err) => {
            debug!(error = %&err, "Error getting category from database");
            Err(util::error(err))
        }
    }
}

pub async fn get_all(db: Database, user: User) -> Result<impl Reply, Rejection> {
    let get_categories = GetCategoriesForUser::new(user.user_id);
    match get_categories.execute(&*db.0) {
        Ok(cts) => {
            let resp: Vec<CategoryResponse> =
                cts.into_iter().map(Category::into_response).collect();
            Ok(reply::json(&resp))
        }
        Err(err) => {
            debug!(error = %&err, "Error getting categories from database");
            return Err(util::error(err));
        }
    }
}
