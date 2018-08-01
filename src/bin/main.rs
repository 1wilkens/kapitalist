extern crate dotenv;
extern crate simplelog;

extern crate actix_web;

extern crate kapitalist;

use kapitalist::{
    api,
    db::DatabaseExecutor,
    state::AppState,
};

use std::env;

use actix_web::{actix, server, App};

fn parse_env() {
    for item in dotenv::dotenv_iter().unwrap() {
        if let Ok((key, val)) = item {
            if let Err(env::VarError::NotPresent) = env::var(&key) {
                env::set_var(&key, &val);
            }
        }
    }
}

fn check_env() -> bool {
    env::var("KAPITALIST_DB").is_ok() && env::var("KAPITALIST_JWT_SECRET").is_ok()
}

fn main() {
    parse_env();
    if !check_env() {
        println!("[CRIT] Failed to validate environment.\nPlease check all required variables are present and valid\nExiting..");
        return;
    }

    let addr = "0.0.0.0:3000";
    let db_url = env::var("KAPITALIST_DB").unwrap();

    let sys = actix::System::new("kapitalist");

    let db = actix::SyncArbiter::start(3, move || {
        DatabaseExecutor::new(&db_url).expect("Failed to instantiate DatabaseExecutor")
    });

    server::new(move || {
        let state = AppState::new(db.clone());
        App::with_state(state)
            .resource("/", |r| r.get().f(api::index))
            .resource("/register", |r| r.post().with(api::user::register))
    }).bind(&addr)
    .expect("Failed to start server")
    .start();

    println!("Started server on: {}", &addr);
    let _ = sys.run();

    // TODO: replace this with actix-web equivalent
    /*rocket::ignite()
        .manage(db::new(&env::var("KAPITALIST_DB").unwrap()))
        .manage(auth::JwtSecret(env::var("KAPITALIST_JWT_SECRET").unwrap()))
        //.catch(errors![err404])
        .mount("/", routes![user::register, user::get_me, user::put_me, user::token])
        .mount("/wallet", routes![wallet::post, wallet::get, wallet::put, wallet::tx_get_all, wallet::tx_post, wallet::tx_get, wallet::tx_put])
        .launch();*/}
