extern crate actix_web;
extern crate dotenv;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate kapitalist;

use kapitalist::{
    api,
    auth::JwtSecret,
    db::DatabaseExecutor,
    log::SlogLogger,
    state::{AppState, Config},
};

use std::env;

use actix_web::{actix, server, App};

fn parse_env() {
    for item in dotenv::dotenv_iter().unwrap() {
        if let Ok((key, val)) = item {
            if let Err(env::VarError::NotPresent) = env::var(&key) {
                env::set_var(&key, &val);
            }
        }
    }
}

fn check_env() -> bool {
    env::var("KAPITALIST_DB").is_ok() && env::var("KAPITALIST_JWT_SECRET").is_ok()
}

fn main() {
    parse_env();
    if !check_env() {
        // XXX: Improve format and print environment variables found and missing
        println!("Failed to validate environment.\nPlease check all required variables are present and valid\nExiting..");
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

    // logging drain
    let drain = make_drain();

    server::new(move || {
        let state = AppState::new(cfg.clone(), db.clone());
        App::with_state(state)
            .middleware(SlogLogger::new(drain.clone()))
            .resource("/", |r| r.get().f(api::index))
            .resource("/register", |r| r.post().with(api::user::register))
            .resource("/token", |r| r.post().with(api::user::token))
            .resource("/me", |r| r.get().with(api::user::get_me))
    }).bind(&addr)
    .expect("Failed to start server")
    .start();

    println!("Started server on: {}", &addr);
    let _ = sys.run();
}

fn make_drain() -> slog::Logger {
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}
