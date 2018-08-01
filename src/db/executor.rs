use actix_web::actix::{Actor, Handler, Message, SyncContext};

use diesel::{
    prelude::*,
};

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

    fn handle(&mut self, msg: GetPGVersion, _: &mut Self::Context) -> Self::Result
    {
        Err(())
    }
}
