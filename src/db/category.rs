use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use slog::trace;

use crate::db::{schema::categories, DatabaseExecutor};
use crate::request::{CategoryCreationRequest, CategoryUpdateRequest};

/// Database entity representing a transaction category
///
/// id        -
/// user_id   -
/// name      -
/// color     -
/// create_at -
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "categories"]
pub struct Category {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub user_id: Option<i64>,
    pub name: String,
    pub color: String,
    pub created_at: NaiveDateTime,
}

/// Insertable database entity to create a new category
///
/// user_id -
/// name    -
/// color   -
#[derive(Debug, Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub parent_id: Option<i64>,
    pub user_id: i64,
    pub name: String,
    pub color: String,
}

/// Actix message to retrieve a category entity from the database
#[derive(Debug)]
pub struct GetCategory {
    pub(crate) cid: i64,
    pub(crate) uid: i64,
}

/// Actix message to update a category entity in the database
#[derive(Debug)]
pub struct UpdateCategory {
    pub uid: i64,
    pub cid: i64,
    pub parent_id: Option<Option<i64>>,
    pub name: Option<String>,
    pub color: Option<String>,
}

/// Actix message to delete a category entity from the database
#[derive(Debug)]
pub struct DeleteCategory {
    pub(crate) cid: i64,
    pub(crate) uid: i64,
}

impl NewCategory {
    pub fn from_request(req: CategoryCreationRequest, uid: i64) -> NewCategory {
        NewCategory {
            parent_id: req.parent_id,
            user_id: uid,
            name: req.name,
            color: req.color,
        }
    }
}

impl Message for NewCategory {
    type Result = Result<Category, Error>;
}

impl Handler<NewCategory> for DatabaseExecutor {
    type Result = Result<Category, Error>;

    fn handle(&mut self, msg: NewCategory, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::categories::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        let category: Category = diesel::insert_into(categories)
            .values(&msg)
            .get_result(&self.0)
            .map_err(error::ErrorInternalServerError)?;

        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?category);
        Ok(category)
    }
}

impl GetCategory {
    pub fn new(category_id: i64, user_id: i64) -> GetCategory {
        GetCategory {
            cid: category_id,
            uid: user_id,
        }
    }
}

impl Message for GetCategory {
    type Result = Result<Option<Category>, Error>;
}

impl Handler<GetCategory> for DatabaseExecutor {
    type Result = Result<Option<Category>, Error>;

    fn handle(&mut self, msg: GetCategory, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::categories::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let category = categories
            .filter(id.eq(&msg.cid))
            .filter(user_id.is_null().or(user_id.eq(&msg.uid)))
            .get_result(&self.0)
            .optional()
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?category);
        Ok(category)
    }
}

impl UpdateCategory {
    pub fn from_request(user_id: i64, category_id: i64, req: CategoryUpdateRequest) -> UpdateCategory {
        UpdateCategory {
            uid: user_id,
            cid: category_id,
            parent_id: req.parent_id,
            name: req.name,
            color: req.color,
        }
    }
}

impl Message for UpdateCategory {
    type Result = Result<Option<Category>, Error>;
}

impl Handler<UpdateCategory> for DatabaseExecutor {
    type Result = Result<Option<Category>, Error>;

    fn handle(&mut self, msg: UpdateCategory, ctx: &mut Self::Context) -> Self::Result {
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let category = self.handle(GetCategory::new(msg.uid, msg.cid), ctx);
        let result = match category {
            Ok(Some(mut c)) => {
                if let Some(pid) = msg.parent_id {
                    c.parent_id = pid;
                }
                if let Some(ref name) = msg.name {
                    c.name = name.clone();
                }
                if let Some(ref color) = msg.color {
                    c.color = color.clone();
                }
                diesel::update(&c)
                    .set(&c)
                    .get_result(&self.0)
                    .optional()
                    .map_err(error::ErrorInternalServerError)?
            }
            _ => None,
        };
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl DeleteCategory {
    pub fn new(user_id: i64, category_id: i64) -> DeleteCategory {
        DeleteCategory {
            cid: category_id,
            uid: user_id,
        }
    }
}

impl Message for DeleteCategory {
    type Result = Result<bool, Error>;
}

impl Handler<DeleteCategory> for DatabaseExecutor {
    type Result = Result<bool, Error>;

    fn handle(&mut self, msg: DeleteCategory, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::categories::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let res = diesel::delete(categories)
            .filter(id.eq(&msg.cid))
            .filter(user_id.is_null().or(user_id.eq(&msg.uid)))
            .execute(&self.0)
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?res);
        Ok(res > 0)
    }
}
