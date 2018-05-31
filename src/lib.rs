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

pub mod api;

pub mod auth;
pub mod model;
pub mod request;
pub mod response;
pub mod schema;
pub mod util;
