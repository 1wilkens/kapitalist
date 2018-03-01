#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
#[macro_use]
extern crate diesel;

extern crate rocket;
extern crate rocket_contrib;

mod db_pool;

use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

fn main() {
    // initialize env
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(db_pool::init(&db_url))
        .mount("/", routes![index])
        .launch();
}