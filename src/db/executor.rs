use actix_web::actix::{Actor, SyncContext};

use diesel::{
    prelude::*,
};

/// The database executor actor
pub struct DatabaseExecutor(pub PgConnection);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

impl DatabaseExecutor {
    pub fn new(db_url: &str) -> Option<DatabaseExecutor> {
        let conn = PgConnection::establish(db_url).ok()?;
        Some(DatabaseExecutor(conn))
    }
}
