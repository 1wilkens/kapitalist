extern crate kapitalist;

use actix_web::{actix, server};
use slog::{error, info, o};

use std::env;

use kapitalist::{build_app, Config};

fn init_logging() -> slog::Logger {
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

fn load_env() {
    for item in dotenv::dotenv_iter().unwrap() {
        if let Ok((key, val)) = item {
            if let Err(env::VarError::NotPresent) = env::var(&key) {
                env::set_var(&key, &val);
            }
        }
    }
}

fn main() {
    // init logging
    let log = init_logging();

    // XXX: add help flag (debate whether to use a crate for this)

    // load and check environment
    load_env();
    if let Err(var) = Config::check_env(&log) {
        error!(&log, "Failed to validate environment"; "missing" => var);
        return;
    }

    // load configuration
    let cfg = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(&log, "Failed to parse configuration"; "error" => ?e);
            return;
        }
    };

    // init actix main system
    let sys = actix::System::new("kapitalist");

    // configure http server
    let cfg_ = cfg.clone();
    let log_ = log.clone();
    let server = server::new(move || build_app(&cfg_, &log_))
        .bind(&cfg.addr)
        .expect(&format!("Failed to bind address from configuration: {}", &cfg.addr));

    // start server
    server.start();
    info!(&log, "Started server on: {}", &cfg.addr);
    let _ = sys.run();
}
