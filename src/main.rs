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

mod db;
mod models;
mod schema;
mod user;
mod util;
mod wallet;

use std::env;

#[get("/")]
fn index() -> &'static str {
    "Kapitalist is running allright!"
}

fn main() {
    // initialize env
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    rocket::ignite()
        .manage(db::new(&db_url))
        .mount("/", routes![index, user::register, user::get_me, user::put_me, user::token])
        .mount("/wallet", routes![wallet::post, wallet::get, wallet::put, wallet::tx_get_all, wallet::tx_post, wallet::tx_get, wallet::tx_put])
        .launch();
}