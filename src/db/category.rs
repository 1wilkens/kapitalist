use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use slog::trace;

use crate::db::{schema::categories, DatabaseExecutor};
use crate::request::CategoryCreationRequest;

/// Database entity representing a transaction category
///
/// id        -
/// user_id   -
/// name      -
/// color     -
/// create_at -
#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Category {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub user_id: Option<i32>,
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
    pub parent_id: Option<i32>,
    pub user_id: i32,
    pub name: String,
    pub color: String,
}

/// Actix message to retrieve a wallet entity from the database
#[derive(Debug)]
pub struct GetCategory {
    pub(crate) cid: i32,
    pub(crate) uid: i32,
}

impl NewCategory {
    pub fn from_request(req: CategoryCreationRequest, uid: i32) -> NewCategory {
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
    pub fn new(category_id: i32, user_id: i32) -> GetCategory {
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
