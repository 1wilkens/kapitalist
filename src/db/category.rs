use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};

use crate::db::{schema::categories, DatabaseExecutor};

#[derive(Debug, Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub user_id: i32,
    pub name: String,
    pub color: String,
}

/// Actix message to retrieve a wallet entity from the database
#[derive(Debug)]
pub struct GetCategory {
    pub(crate) cid: i32,
    pub(crate) uid: Option<i32>,
}
