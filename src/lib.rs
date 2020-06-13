//#![deny(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_option,
    clippy::redundant_field_names
)]

#[macro_use]
extern crate diesel;

pub mod api;
pub mod auth;
pub mod cfg;
pub mod cron;
pub mod db;
pub mod err;
pub mod state;

//mod log;
pub use crate::cfg::Config;
pub use crate::state::AppState;

use std::sync::Arc;

use serde::de::DeserializeOwned;
use warp::{filters::BoxedFilter, log::Info, Filter, Rejection, Reply};

const APPLICATION_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn build_site(state: Arc<AppState>, db: crate::db::PgPool) -> BoxedFilter<(impl Reply,)> {
    // static endpoints
    let index = warp::get().and_then(api::index);
    let version = warp::path!("version")
        .and(warp::get())
        .and_then(api::version);

    let static_eps = version.or(index);

    // /register
    let register = warp::path!("register")
        .and(db::attach(db.clone()))
        .and(with_json_body())
        .and(warp::post())
        .and_then(api::user::register);

    // /me
    let me = warp::path!("me");
    let get_me = me
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::get())
        .and_then(api::user::get_me);
    let put_me = me
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(with_json_body())
        .and(warp::put())
        .and_then(api::user::put_me);

    // /token
    let token = warp::path!("token")
        .and(state::attach(state.clone()))
        .and(db::attach(db.clone()))
        .and(with_json_body())
        .and(warp::post())
        .and_then(api::user::token);

    let user = register.or(get_me).or(put_me).or(token);

    // /wallet
    let wallet_base = warp::path!("wallet");
    let post = wallet_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(with_json_body())
        .and(warp::post())
        .and_then(api::wallet::post);
    let get = wallet_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(warp::get())
        .and_then(api::wallet::get);
    let put = wallet_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(with_json_body())
        .and(warp::put())
        .and_then(api::wallet::put);
    let delete = wallet_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(warp::delete())
        .and_then(api::wallet::delete);

    // /wallet/all
    let all = wallet_base
        .and(warp::path("all").and(warp::path::end()))
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::get())
        .and_then(api::wallet::all);

    let wallet = post.or(get).or(put).or(delete).or(all);

    // /category
    let category_base = warp::path!("category");
    let post = category_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(with_json_body())
        .and(warp::post())
        .and_then(api::category::post);
    let get = category_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(warp::get())
        .and_then(api::category::get);
    let put = category_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(with_json_body())
        .and(warp::put())
        .and_then(api::category::put);
    let delete = category_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(warp::delete())
        .and_then(api::category::delete);

    // /category/all
    let all = category_base
        .and(warp::path("all").and(warp::path::end()))
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::get())
        .and_then(api::wallet::all);

    let category = post.or(get).or(put).or(delete).or(all);

    // /transaction
    let tx_base = warp::path!("transaction");
    let post = tx_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(with_json_body())
        .and(warp::post())
        .and_then(api::transaction::post);
    let get = tx_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(warp::get())
        .and_then(api::transaction::get);
    let put = tx_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(with_json_body())
        .and(warp::put())
        .and_then(api::transaction::put);
    let delete = tx_base
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(warp::delete())
        .and_then(api::transaction::delete);

    // /transaction/all
    let all = tx_base
        .and(warp::path("all").and(warp::path::end()))
        .and(db::attach(db.clone()))
        .and(auth::check(state.clone()))
        .and(warp::path!(i64).and(warp::path::end()))
        .and(warp::get())
        .and_then(api::transaction::all);

    let tx = post.or(get).or(put).or(delete).or(all);

    // dynamic endpoints
    let dynamic_eps = user.or(wallet).or(category).or(tx);

    dynamic_eps
        .or(static_eps)
        .recover(err::handle_rejection)
        .with(warp::log::custom(via_tracing))
        .boxed()
}

fn via_tracing(info: Info) {
    tracing::info!(
        "{} \"{} {} {:?}\" {} \"{}\" \"{}\" {:?}",
        OptFmt(info.remote_addr()),
        info.method(),
        info.path(),
        info.version(),
        info.status().as_u16(),
        OptFmt(info.referer()),
        OptFmt(info.user_agent()),
        info.elapsed()
    );
}

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// Taken from warp source as it is sadly not public =/
struct OptFmt<T>(Option<T>);

impl<T: std::fmt::Display> std::fmt::Display for OptFmt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(ref t) = self.0 {
            std::fmt::Display::fmt(t, f)
        } else {
            f.write_str("-")
        }
    }
}
