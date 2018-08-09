use actix_web::{
    actix::{Actor, Handler, Message, SyncContext},
    error::{self, Error},
};

use diesel::{self, prelude::*};

use super::model::{NewUser, User};

/// The database executor actor
pub struct DatabaseExecutor(PgConnection);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

impl DatabaseExecutor {
    pub fn new(db_url: &str) -> Option<DatabaseExecutor> {
        let conn = PgConnection::establish(db_url).ok()?;
        Some(DatabaseExecutor(conn))
    }
}

pub struct GetPGVersion;

impl Message for GetPGVersion {
    type Result = Result<String, Error>;
}

impl Handler<GetPGVersion> for DatabaseExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, _msg: GetPGVersion, _: &mut Self::Context) -> Self::Result {
        Ok("".to_string())
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
