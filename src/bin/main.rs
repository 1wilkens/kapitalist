#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate dotenv;
#[macro_use]
extern crate rocket;

extern crate kapitalist;

use kapitalist::api::{user, wallet};
use kapitalist::{auth, db};

use std::env;

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
        .mount("/", routes![user::register, user::get_me, user::put_me, user::token])
        .mount("/wallet", routes![wallet::post, wallet::get, wallet::put, wallet::tx_get_all, wallet::tx_post, wallet::tx_get, wallet::tx_put])
        .launch();
}