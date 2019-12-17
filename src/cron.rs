use diesel::{Connection, PgConnection};
use slog::error;

use crate::state;

pub fn execute(config: &state::Config, log: &slog::Logger) -> Result<(), String> {
    let _conn = PgConnection::establish(&config.db_url).map_err(|_| "Failed to establish connection to database")?;
    error!(&log, "That command is currently not implemented!");
    Err("Not implemented yet".into())
}
