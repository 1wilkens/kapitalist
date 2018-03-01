#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
#[macro_use]
extern crate diesel;

extern crate rocket;
extern crate rocket_contrib;

mod db_pool;
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

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(db_pool::init(&db_url))
        .mount("/", routes![index])
        .mount("/wallets", routes![wallet::get, wallet::get_one, wallet::post, wallet::put])
        .launch();
}