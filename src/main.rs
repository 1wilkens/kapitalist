#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate chrono;

extern crate dotenv;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

extern crate rocket;
extern crate rocket_contrib;

extern crate jsonwebtoken as jwt;

extern crate ring_pwhash as pwhash;

mod endpoint;

mod auth;
mod db;
mod model;
mod request;
mod response;
mod schema;
mod util;

use endpoint::{user, wallet};

use std::env;

#[get("/")]
fn index() -> &'static str {
    "Kapitalist is running allright!"
}

/*#[error(404)]
fn err404(req: &::rocket::request::Request) -> &'static str {
    "404"
}*/

fn setup_env() {
    for item in dotenv::dotenv_iter().unwrap() {
        let (key, val) = item.unwrap();
        if let Err(env::VarError::NotPresent) = env::var(&key) {
            env::set_var(&key, &val);
        }
    }
}

fn main() {
    // initialize env
    setup_env();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    rocket::ignite()
        .manage(db::new(&db_url))
        //.catch(errors![err404])
        .mount("/", routes![index, user::register, user::get_me, user::put_me, user::token])
        .mount("/wallet", routes![wallet::post, wallet::get, wallet::put, wallet::tx_get_all, wallet::tx_post, wallet::tx_get, wallet::tx_put])
        .launch();
}