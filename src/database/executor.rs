use actix_web::actix::{Actor, SyncContext};

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

/// The database executor actor
pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}
