extern crate kapitalist;

use actix_web::{actix, server, App};
use slog::{debug, error, info, o, trace, Logger};

use std::collections::HashMap;
use std::env;

use kapitalist::{
    api,
    db::DatabaseExecutor,
    log::SlogLogger,
    state::{AppState, Config},
};

/// Required environment variables for kapitalist
///
/// - KAPITALIST_HOST       - Which IP address to listen on
/// - KAPITALIST_PORT       - Which port to listen on
/// - KAPITALIST_DB         - Connection string of the backing database (diesel format)
/// - KAPITALIST_JWT_SECRET - JWT secret to sign tokens with
static REQUIRED_VARIABLES: [&str; 4] = [
    "KAPITALIST_HOST",
    "KAPITALIST_PORT",
    "KAPITALIST_DB",
    "KAPITALIST_JWT_SECRET",
];

fn load_env() {
    for item in dotenv::dotenv_iter().unwrap() {
        if let Ok((key, val)) = item {
            if let Err(env::VarError::NotPresent) = env::var(&key) {
                env::set_var(&key, &val);
            }
        }
    }
}

fn check_env(log: &Logger) -> Result<(), String> {
    trace!(log, "Checking environment");
    let vars: HashMap<String, String> = env::vars().collect();

    for v in REQUIRED_VARIABLES.iter() {
        if vars.contains_key(*v) && !vars[*v].is_empty() {
            debug!(log, "Found required env variable";
                "value" => &vars[*v],
                "name" => v);
        } else {
            debug!(log, "Missing required env variables"; "name" => v);
            return Err(v.to_string());
        }
    }

    Ok(())
}

fn main() {
    // logging log
    let log = init_logging();

    // XXX: add help flag (debate whether to use a crate for this)

    // load and check environment
    load_env();
    if let Err(var) = check_env(&log) {
        error!(log, "Failed to validate environment"; "missing" => var);
        return;
    }

    // load configuration
    let cfg = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(log, "Failed to parse configuration"; "error" => ?e);
            return;
        }
    };

    // actix main system
    let sys = actix::System::new("kapitalist");

    // database connection
    let url = cfg.db_url.clone();
    let log_ = log.clone();
    let db = actix::SyncArbiter::start(3, move || {
        // XXX: Need to check for errors here somehow. Currently the actor thread just panicks
        DatabaseExecutor::new(&url, log_.clone()).expect("Failed to instantiate DatabaseExecutor")
    });

    let log_ = log.clone();
    let cfg_ = cfg.clone();
    server::new(move || {
        let state = AppState::new(cfg_.clone(), db.clone())
            .with_logger(log_.clone())
            .build();
        App::with_state(state)
            .middleware(SlogLogger::new(log_.clone()))
            .resource("/", |r| r.get().f(api::index))
            .resource("/register", |r| r.post().with(api::user::register))
            .resource("/token", |r| r.post().with(api::user::token))
            .resource("/me", |r| r.get().with(api::user::get_me))
            .resource("/wallet", |r| r.post().with(api::wallet::post))
            .resource("/wallet/{id}", |r| r.get().with(api::wallet::get))
            .resource("/category", |r| r.post().with(api::category::post))
    })
    .bind(&cfg.addr)
    .expect("Failed to start server")
    .start();

    info!(log, "Started server on: {}", &cfg.addr);
    let _ = sys.run();
}

fn init_logging() -> slog::Logger {
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}
