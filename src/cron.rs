use diesel::{Connection, PgConnection};
use tracing::error;

use crate::cfg::Config;

pub fn execute(config: &Config) -> Result<(), String> {
    let _conn = PgConnection::establish(&config.db_url)
        .map_err(|_| "Failed to establish connection to database")?;
    error!("That command is currently not implemented!");
    Err("Not implemented yet".into())
}
