use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use slog::trace;

use kapitalist_types::request::UserCreationRequest;

use crate::db::{schema::users, DatabaseExecutor};

/// Database entity representing a user account
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable, AsChangeset)]
pub struct User {
    /// Account Id
    pub id: i64,
    /// User's current email address
    pub email: String,
    /// Salt and hash of the user's password
    pub secret: String,
    /// User's current username
    pub username: String,
    /// Creation date of the user's account
    pub created_at: NaiveDateTime,
}

/// Insertable database entity to create new user accounts
#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    /// The email address used to register
    pub email: String,
    /// Salt and hash of the password used to register
    pub secret: String,
    /// The username used to register
    pub username: String,
}

/// Actix message to retrieve a user entity from the database
/// XXX: Change this to take Options of i64 and String to allow for multiple selection methods
#[derive(Debug)]
pub struct GetUser {
    pub email: String,
}

impl NewUser {
    /// XXX: This should return a result, figure out fitting error type
    pub fn from_request(req: UserCreationRequest) -> Option<Self> {
        use libreauth::pass::HashBuilder;

        let hasher = HashBuilder::new().finalize().expect("[CRIT] Failed to create Hasher");
        // XXX: Should handle hash errors here
        let hash = hasher.hash(&req.password).ok()?;
        // XXX: This looks rather ugly, but unwrap_or_else tries to move req
        let name = if let Some(name) = req.name {
            name
        } else {
            req.email.clone()
        };
        Some(Self {
            email: req.email,
            secret: hash,
            username: name,
        })
    }
}

impl Message for NewUser {
    type Result = Result<Option<User>, Error>;
}

impl Handler<NewUser> for DatabaseExecutor {
    type Result = Result<Option<User>, Error>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        let exists: bool = diesel::select(diesel::dsl::exists(users.filter(email.eq(&msg.email))))
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error getting User from Db"))?;

        if exists {
            return Ok(None);
        }

        let user = diesel::insert_into(users)
            .values(&msg)
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error inserting user"))?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?user);
        Ok(Some(user))
    }
}

impl Message for GetUser {
    type Result = Result<Option<User>, Error>;
}

impl Handler<GetUser> for DatabaseExecutor {
    type Result = Result<Option<User>, Error>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        let user = users
            .filter(email.eq(&msg.email))
            .get_result(&self.0)
            .optional()
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?user);
        Ok(user)
    }
}
