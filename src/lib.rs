#![deny(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions,
    clippy::option_option,
    clippy::redundant_field_names
)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;

pub mod auth;
pub mod state;

//mod log;

pub use crate::state::Config;

/// Construct and configure an instance of an `rocket::Rocket` from the given `kapitalist::Config` and
/// `slog::Logger`
pub fn build_rocket(config: &state::Config, log: &slog::Logger) -> rocket::Rocket {
    use rocket::config::Environment;
    // database connection
    let db = db::build_config(&config.db_url);

    let log_ = log.clone();
    let cfg_ = config.clone();

    let state = state::AppState::new(cfg_.clone()).with_logger(log_.clone()).build();

    let config = rocket::Config::build(Environment::Development)
        .address(&config.address)
        .port(config.port)
        .extra("databases", db)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(state)
        .attach(db::Database::fairing())
        .mount(
            "/",
            routes![
                // Kapitalist related
                api::index,
                api::version,
                // User management
                api::user::get_me,
                api::user::put_me,
                api::user::register,
                api::user::token
            ],
        )
        .mount(
            "/wallet",
            routes![
                // Wallet management
                api::wallet::post,
                api::wallet::get_all,
                api::wallet::get,
                api::wallet::put,
                api::wallet::delete
            ],
        )
        .mount(
            "/category",
            routes![
                // Category management
                api::category::post,
                api::category::get_all,
                api::category::get,
                api::category::put,
                api::category::delete
            ],
        )
        .mount(
            "/transaction",
            routes![
                // Transaction management
                api::transaction::post,
                api::transaction::get_all,
                api::transaction::get,
                api::transaction::put,
                api::transaction::delete
            ],
        )
}
