use actix_web::actix::{Actor, Handler, Message, SyncContext};

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
    type Result = Result<String, ()>;
}

impl Handler<GetPGVersion> for DatabaseExecutor {
    type Result = Result<String, ()>;

    fn handle(&mut self, msg: GetPGVersion, _: &mut Self::Context) -> Self::Result {
        Err(())
    }
}

impl Message for NewUser {
    type Result = Result<User, String>;
}

impl Handler<NewUser> for DatabaseExecutor {
    type Result = Result<User, String>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::users::dsl::*;

        // XXX: Figure out error type to be used here and add conversion functions for convenience
        let exists: bool = diesel::select(diesel::dsl::exists(users
            .filter(email.eq(&msg.email))))
            .get_result(&self.0)
            .map_err(|_| "Database error".to_string())?;

        if exists {
            // TODO: should we really return this message?
            return Err("User already exists".into());
        }

        let user: User = diesel::insert_into(users)
            .values(&msg)
            .get_result(&self.0)
            .map_err(|_| "Database error".to_string())?;
        Ok(user)
    }
}
