use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};

use db::{schema::users, DatabaseExecutor};
use request::{UserCreationRequest, UserUpdateRequest};
use util;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub secret_hash: String,
    pub username: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub secret_hash: String,
    pub username: String,
}

impl NewUser {
    pub fn from_request(req: UserCreationRequest) -> NewUser {
        use pwhash::scrypt::scrypt_simple;

        let params = util::get_scrypt_params();
        // Unwrap is safe here because scrypt_simple does not ever return an error
        let hash = scrypt_simple(&req.password, &params).unwrap();
        NewUser {
            email: req.email,
            secret_hash: hash,
            username: req.name,
        }
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
            .map_err(|_| error::ErrorInternalServerError("Error getting user"))?;

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

pub struct GetUser(pub String);

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
