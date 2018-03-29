#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;
extern crate dotenv;

extern crate chrono;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

// "Steal" rocket's logging macros
#[macro_use]
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

fn parse_env() {
    for item in dotenv::dotenv_iter().unwrap() {
        if let Ok((key, val)) = item {
            if let Err(env::VarError::NotPresent) = env::var(&key) {
                env::set_var(&key, &val);
            }
        }
    }
}

fn check_env() -> bool{
    env::var("KAPITALIST_DB").is_ok()
    && env::var("KAPITALIST_JWT_SECRET").is_ok()
}

fn main() {
    parse_env();
    if !check_env() {
        println!("[CRIT] Failed to validate environment.\nPlease check all required variables are present and valid\nExiting..");
        return;
    }

    rocket::ignite()
        .manage(db::new(&env::var("KAPITALIST_DB").unwrap()))
        .manage(auth::JwtSecret(env::var("KAPITALIST_JWT_SECRET").unwrap()))
        //.catch(errors![err404])
        .mount("/", routes![index, user::register, user::get_me, user::put_me, user::token])
        .mount("/wallet", routes![wallet::post, wallet::get, wallet::put, wallet::tx_get_all, wallet::tx_post, wallet::tx_get, wallet::tx_put])
        .launch();
}