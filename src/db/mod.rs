use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

use warp::{Filter, Rejection};

mod schema;

pub mod category;
pub mod transaction;
pub mod user;
pub mod wallet;

pub struct Database(pub PooledConnection<ConnectionManager<PgConnection>>);

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

pub fn attach(pool: PgPool) -> impl Filter<Extract = (Database,), Error = Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {
            match pool.get() {
                Ok(conn) => Ok(Database(conn)),
                // FIXME: replace with proper error type
                Err(err) => Err(warp::reject::reject()),
            }
        })
}
