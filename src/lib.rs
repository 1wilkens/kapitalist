#![deny(clippy::pedantic)]
#![allow(
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

    let state = state::AppState::new(cfg_.clone())
        .with_logger(log_.clone())
        .build();

    let config = rocket::Config::build(Environment::Development)
        .address(&config.address)
        .port(config.port)
        .extra("databases", db)
        .finalize()
        .unwrap();

    let rocket = rocket::custom(config)
        .manage(state)
        .attach(db::Database::fairing())
        .mount("/", routes![api::index, api::version])
        .mount("/", routes![api::user::register]);
    rocket

    /*actix_web::App::with_state(state)
        .middleware(log::SlogMiddleware::new(log_.clone()))
        // User management
        .resource("/", |r| r.get().f(api::index))
        .resource("/version", |r| r.get().f(api::version))
        .resource("/register", |r| r.post().with(api::user::register))
        .resource("/token", |r| r.post().with(api::user::token))
        .resource("/me", |r| r.get().with(api::user::get_me))
        .resource("/me", |r| r.put().with(api::user::put_me))
        // Wallets
        .resource("/wallet", |r| r.post().with(api::wallet::post))
        .resource("/wallet/all", |r| r.get().with(api::wallet::get_all))
        .resource("/wallet/{id}", |r| r.get().with(api::wallet::get))
        .resource("/wallet/{id}", |r| r.put().with(api::wallet::put))
        .resource("/wallet/{id}", |r| r.delete().with(api::wallet::delete))
        // Categories
        .resource("/category", |r| r.post().with(api::category::post))
        .resource("/category/{id}", |r| r.get().with(api::category::get))
        .resource("/category/{id}", |r| r.put().with(api::category::put))
        .resource("/category/{id}", |r| r.delete().with(api::category::delete))
        // Transactions
        .resource("/transaction", |r| r.post().with(api::transaction::post))
        .resource("/transaction/all/{id}", |r| r.get().with(api::transaction::get_all))
        .resource("/transaction/{id}", |r| r.get().with(api::transaction::get))
        .resource("/transaction/{id}", |r| r.put().with(api::transaction::put))
        .resource("/transaction/{id}", |r| r.delete().with(api::transaction::delete))*/
}
