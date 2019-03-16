use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use slog::trace;

use kapitalist_types::request::{UserCreationRequest, UserUpdateRequest};
use kapitalist_types::response::UserResponse;

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
    /// Creation timestamp of the user's account
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
#[derive(Debug)]
pub struct GetUser {
    pub uid: Option<i64>,
    pub email: Option<String>,
}

/// Actix message to update a user entity in the database
#[derive(Debug)]
pub struct UpdateUser {
    pub uid: i64,
    pub email: Option<String>,
    pub secret: Option<String>,
    pub username: Option<String>,
}

impl User {
    pub fn into_response(self) -> UserResponse {
        UserResponse {
            email: self.email,
            username: self.username,
            created_at: self.created_at,
        }
    }
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

impl GetUser {
    /// Get the user with the given Id
    pub fn by_id(uid: i64) -> Self {
        Self {
            uid: Some(uid),
            email: None,
        }
    }

    /// Get the user with the given Email address
    pub fn by_email(email: String) -> Self {
        Self {
            uid: None,
            email: Some(email),
        }
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

        if msg.uid.is_none() && msg.email.is_none() {
            // XXX: Fix error message?
            let err = error::ErrorInternalServerError("Invalid GetUser object");
            trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?err);
            return Err(err);
        }

        let user = match (msg.uid, &msg.email) {
            // Get by Id
            (Some(uid), None) => users
                .filter(id.eq(&uid))
                .get_result(&self.0)
                .optional()
                .map_err(error::ErrorInternalServerError)?,
            // Get by email
            (None, Some(ref email_)) => users
                .filter(email.eq(email_))
                .get_result(&self.0)
                .optional()
                .map_err(error::ErrorInternalServerError)?,
            _ => unreachable!(),
        };

        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?user);
        Ok(user)
    }
}

impl UpdateUser {
    pub fn from_request(user_id: i64, req: UserUpdateRequest) -> Self {
        Self {
            uid: user_id,
            email: req.email,
            secret: req.password,
            username: req.name,
        }
    }
}

impl Message for UpdateUser {
    type Result = Result<Option<User>, Error>;
}

impl Handler<UpdateUser> for DatabaseExecutor {
    type Result = Result<Option<User>, Error>;

    fn handle(&mut self, msg: UpdateUser, ctx: &mut Self::Context) -> Self::Result {
        let user = self.handle(GetUser::by_id(msg.uid), ctx);
        let result = match user {
            Ok(Some(mut u)) => {
                if let Some(ref email) = msg.email {
                    u.email = email.clone();
                }
                if let Some(ref secret) = msg.secret {
                    // XXX: Validate password hash here?
                    u.secret = secret.clone();
                }
                if let Some(ref username) = msg.username {
                    u.username = username.clone()
                }
                diesel::update(&u)
                    .set(&u)
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
