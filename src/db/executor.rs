use actix_web::{
    actix::{Actor, Handler, Message, SyncContext},
    error::Error,
};
use diesel::{pg::PgConnection, Connection};
use slog::Logger;

/// The database executor actor
pub struct DatabaseExecutor(pub(crate) PgConnection, pub(crate) Logger);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

impl DatabaseExecutor {
    pub fn new(db_url: &str, log: Logger) -> Option<Self> {
        let conn = PgConnection::establish(db_url).ok()?;
        Some(DatabaseExecutor(conn, log))
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
