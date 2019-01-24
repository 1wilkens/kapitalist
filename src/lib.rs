#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;

pub mod auth;
pub mod log;
pub mod request;
pub mod response;
pub mod state;

use slog::Logger;

pub use crate::state::Config;

pub fn build_app(config: &state::Config, log: &Logger) -> actix_web::App<state::AppState> {
    // database connection
    let url = config.db_url.clone();
    let log_ = log.clone();
    let db = actix::SyncArbiter::start(3, move || {
        // XXX: Need to check for errors here somehow. Currently the actor thread just panicks
        db::DatabaseExecutor::new(&url, log_.clone()).expect("Failed to instantiate DatabaseExecutor")
    });

    let log_ = log.clone();
    let cfg_ = config.clone();

    let state = state::AppState::new(cfg_.clone(), db.clone())
        .with_logger(log_.clone())
        .build();

    actix_web::App::with_state(state)
        .middleware(log::SlogMiddleware::new(log_.clone()))
        // User management
        .resource("/", |r| r.get().f(api::index))
        .resource("/register", |r| r.post().with(api::user::register))
        .resource("/token", |r| r.post().with(api::user::token))
        .resource("/me", |r| r.get().with(api::user::get_me))
        // Wallets
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
        .resource("/transaction/{id}", |r| r.delete().with(api::transaction::delete))
}
