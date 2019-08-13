use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
//use slog::trace;

use kapitalist_types::request::{CategoryCreationRequest, CategoryUpdateRequest};
use kapitalist_types::response::CategoryResponse;

use crate::db::schema::categories;

/// Database entity representing a transaction category
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
#[derive(Debug, Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub parent_id: Option<i64>,
    pub user_id: i64,
    pub name: String,
    pub color: String,
}

/// Struct to retrieve a category entity from the database
#[derive(Debug)]
pub struct GetCategory {
    pub cid: i64,
    pub uid: i64,
}

/// Struct to update a category entity in the database
#[derive(Debug)]
pub struct UpdateCategory {
    pub uid: i64,
    pub cid: i64,
    pub parent_id: Option<Option<i64>>,
    pub name: Option<String>,
    pub color: Option<String>,
}

/// Struct to delete a category entity from the database
#[derive(Debug)]
pub struct DeleteCategory {
    pub cid: i64,
    pub uid: i64,
}

impl Category {
    pub fn into_response(self) -> CategoryResponse {
        CategoryResponse {
            id: self.id,
            parent_id: self.parent_id,
            user_id: self.user_id,
            name: self.name,
            color: self.color,
            created_at: self.created_at,
        }
    }
}

impl NewCategory {
    pub fn from_request(req: CategoryCreationRequest, uid: i64) -> Self {
        Self {
            parent_id: req.parent_id,
            user_id: uid,
            name: req.name,
            color: req.color,
        }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<Category, &'static str> {
        use crate::db::schema::categories::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        let category: Category = diesel::insert_into(categories)
            .values(self)
            .get_result(conn)
            .map_err(|_| "Error inserting Category into database")?;

        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?category);
        Ok(category)
    }
}

impl GetCategory {
    pub fn new(category_id: i64, user_id: i64) -> Self {
        Self {
            cid: category_id,
            uid: user_id,
        }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<Option<Category>, &'static str> {
        use crate::db::schema::categories::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let category = categories
            .filter(id.eq(self.cid))
            .filter(user_id.is_null().or(user_id.eq(self.uid)))
            .get_result(conn)
            .optional()
            .map_err(|_| "Error getting Category from database")?;
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?category);
        Ok(category)
    }
}

impl UpdateCategory {
    pub fn from_request(user_id: i64, category_id: i64, req: CategoryUpdateRequest) -> Self {
        Self {
            uid: user_id,
            cid: category_id,
            parent_id: req.parent_id,
            name: req.name,
            color: req.color,
        }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<Option<Category>, &'static str> {
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let category = GetCategory::new(self.uid, self.cid).execute(conn);
        let result = match category {
            Ok(Some(mut c)) => {
                if let Some(pid) = self.parent_id {
                    c.parent_id = pid;
                }
                if let Some(ref name) = self.name {
                    c.name = name.clone();
                }
                if let Some(ref color) = self.color {
                    c.color = color.clone();
                }
                diesel::update(&c)
                    .set(&c)
                    .get_result(conn)
                    .optional()
                    .map_err(|_| "Error updating Category in database")?
            }
            _ => None,
        };
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl DeleteCategory {
    pub fn new(user_id: i64, category_id: i64) -> Self {
        Self {
            cid: category_id,
            uid: user_id,
        }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<bool, &'static str> {
        use crate::db::schema::categories::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let res = diesel::delete(categories)
            .filter(id.eq(self.cid))
            .filter(user_id.is_null().or(user_id.eq(self.uid)))
            .execute(conn)
            .map_err(|_| "Error deleting Category from database")?;
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?res);
        Ok(res > 0)
    }
}
