extern crate actix_web;
extern crate dotenv;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate kapitalist;

use kapitalist::{
    api,
    db::DatabaseExecutor,
    log::SlogLogger,
    state::{AppState, Config},
};

use std::collections::HashMap;
use std::env;

use actix_web::{actix, server, App};
use slog::Logger;

/// Required environment variables for kapitalist
///
/// - "KAPITALIST_HOST"       - Which IP address to listen on
/// - "KAPITALIST_PORT"       - Which port to listen on
/// - "KAPITALIST_DB"         - Connection string of the backing database (diesel format)
/// - "KAPITALIST_JWT_SECRET" - JWT secret to sign tokens with
static REQUIRED_VARIABLES: [&str; 4] = [
    "KAPITALIST_HOST",
    "KAPITALIST_PORT",
    "KAPITALIST_DB",
    "KAPITALIST_JWT_SECRET"
];

fn parse_env() {
    for item in dotenv::dotenv_iter().unwrap() {
        if let Ok((key, val)) = item {
            if let Err(env::VarError::NotPresent) = env::var(&key) {
                env::set_var(&key, &val);
            }
        }
    }
}

fn check_env(log: &Logger) -> bool {
    trace!(log, "Cheking environment");
    let vars: HashMap<String, String> = env::vars().collect();

    for v in REQUIRED_VARIABLES.iter() {
        if vars.contains_key(*v) && !vars[*v].is_empty() {
            debug!(log, "Found required env variable"; "name" => v, "value" => &vars[*v]);
        } else {
            debug!(log, "Missing required env variables"; "name" => v);
            return false;
        }
    }

    true
}

fn main() {
    // logging log
    let log = init_logging();

    parse_env();
    if !check_env(&log) {
        // XXX: Improve format and print environment variables found and missing
        error!(log, "Failed to validate environment.\nPlease check all required variables are present and valid\nExiting..");
        return;
    }

    let addr = "0.0.0.0:3000";
    let db_url = env::var("KAPITALIST_DB").unwrap();

    // actix main system
    let sys = actix::System::new("kapitalist");

    // configuration + database connection
    let cfg = Config::from_env().unwrap();
    let db = actix::SyncArbiter::start(3, move || {
        DatabaseExecutor::new(&db_url).expect("Failed to instantiate DatabaseExecutor")
    });

    let tmp = log.clone();
    server::new(move || {
        let state = AppState::new(cfg.clone(), db.clone());
        App::with_state(state)
            .middleware(SlogLogger::new(tmp.clone()))
            .resource("/", |r| r.get().f(api::index))
            .resource("/register", |r| r.post().with(api::user::register))
            .resource("/token", |r| r.post().with(api::user::token))
            .resource("/me", |r| r.get().with(api::user::get_me))
    }).bind(&addr)
    .expect("Failed to start server")
    .start();

    info!(log, "Started server on: {}", &addr);
    let _ = sys.run();
}

fn init_logging() -> slog::Logger {
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}
