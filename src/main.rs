#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate dotenv;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

extern crate rocket;
extern crate rocket_contrib;

extern crate jsonwebtoken as jwt;

mod db_pool;

mod transaction;
mod user;
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
        .manage(db_pool::init(&db_url))
        .mount("/", routes![index])
        .mount("/wallet", routes![wallet::post, wallet::get, wallet::put])
        .mount("/transaction", routes![transaction::post, transaction::get, transaction::put])
        .launch();
}