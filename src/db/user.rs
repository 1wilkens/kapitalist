use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};

use db::{schema::users, DatabaseExecutor};
use request::{UserCreationRequest, /*UserUpdateRequest*/};

/// Database entity representing a user account
///
/// id         - database id
/// email      - user's current email address
/// secret     - salt and hash of the user's password
/// username   - user's current username
/// created_at - creation date of the user account
#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub secret: String,
    pub username: String,
    pub created_at: NaiveDateTime,
}

/// Insertable database entity to create new user accounts
///
/// email    - user's email address used to register
/// secret   - salt and hash of the user's password
/// username - user's chosen username used to register
#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub secret: String,
    pub username: String,
}

/// Actix message to retrieve a user entity from the database
pub struct GetUser(pub String);


impl NewUser {
    /// XXX: This should return a result, figure out fitting error type
    pub fn from_request(req: UserCreationRequest) -> Option<NewUser> {
        use libreauth::pass::HashBuilder;

        let hasher = HashBuilder::new().finalize().expect("[CRIT] Failed to create Hasher");
        // XXX: Should handle hash errors here
        let hash = hasher.hash(&req.password).ok()?;
        Some(NewUser {
            email: req.email,
            secret: hash,
            username: req.name,
        })
    }
}

impl Message for NewUser {
    type Result = Result<User, Error>;
}

impl Handler<NewUser> for DatabaseExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;

        // XXX: Figure out error type to be used here and add conversion functions for convenience
        let exists: bool = diesel::select(diesel::dsl::exists(users.filter(email.eq(&msg.email))))
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error getting User from Db"))?;

        if exists {
            // TODO: should we really return this message?
            return Err(error::ErrorUnauthorized("User already exists"));
        }

        let user: User = diesel::insert_into(users)
            .values(&msg)
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error inserting user"))?;
        Ok(user)
    }
}


impl Message for GetUser {
    type Result = Result<User, Error>;
}

impl Handler<GetUser> for DatabaseExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;

        // XXX: Figure out error type to be used here and add conversion functions for convenience
        let user: User = users
            .filter(email.eq(&msg.0))
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error getting user`"))?;
        Ok(user)
    }
}
