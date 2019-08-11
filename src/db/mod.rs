mod schema;

pub mod category;
//pub mod transaction;
pub mod user;
pub mod wallet;

use rocket::config::Value;
use rocket_contrib::databases::diesel;

use std::collections::HashMap;

#[database("kapitalist")]
pub struct Database(diesel::PgConnection);

pub fn build_config(db_url: &str) -> HashMap<&'static str, Value> {
    let mut db_cfg = HashMap::new();
    let mut dbs = HashMap::new();

    db_cfg.insert("url", Value::from(db_url));
    dbs.insert("kapitalist", Value::from(db_cfg));

    dbs
}
