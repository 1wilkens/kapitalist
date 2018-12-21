use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
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
    pub user_id: Option<i32>,
    pub name: String,
    pub color: Option<String>,
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
    pub user_id: i32,
    pub name: String,
    pub color: Option<String>,
}

/// Actix message to retrieve a wallet entity from the database
#[derive(Debug)]
pub struct GetCategory {
    pub(crate) cid: i32,
    pub(crate) uid: Option<i32>,
}

impl NewCategory {
    pub fn from_request(req: CategoryCreationRequest, uid: i32) -> NewCategory {
        NewCategory {
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
