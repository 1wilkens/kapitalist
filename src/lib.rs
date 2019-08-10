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
extern crate diesel;

pub mod api;
//pub mod db;

pub mod auth;
pub mod state;

//mod log;

pub use crate::state::Config;

/// Construct and configure an instance of an `rocket::Rocket` from the given `kapitalist::Config` and
/// `slog::Logger`
pub fn build_app(config: &state::Config, log: &slog::Logger) -> rocket::Rocket {
    // database connection
    let url = config.db_url.clone();
    let log_ = log.clone();

    let log_ = log.clone();
    let cfg_ = config.clone();

    let state = state::AppState::new(cfg_.clone())
        .with_logger(log_.clone())
        .build();

    let rocket = rocket::ignite()
        .mount("/", routes![api::index]);
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
        .resource("/wallets", |r| r.get().with(api::wallet::get_all))
        .resource("/wallet", |r| r.post().with(api::wallet::post))
        .resource("/wallet/{id}", |r| r.get().with(api::wallet::get))
        .resource("/wallet/{id}", |r| r.put().with(api::wallet::put))
        .resource("/wallet/{id}", |r| r.delete().with(api::wallet::delete))
        // Categories
        .resource("/category", |r| r.post().with(api::category::post))
        .resource("/category/{id}", |r| r.get().with(api::category::get))
        .resource("/category/{id}", |r| r.put().with(api::category::put))
        .resource("/category/{id}", |r| r.delete().with(api::category::delete))
        // Transactions
        .resource("/transactions/{id}", |r| r.get().with(api::transaction::get_all))
        .resource("/transaction", |r| r.post().with(api::transaction::post))
        .resource("/transaction/{id}", |r| r.get().with(api::transaction::get))
        .resource("/transaction/{id}", |r| r.put().with(api::transaction::put))
        .resource("/transaction/{id}", |r| r.delete().with(api::transaction::delete))*/
}
