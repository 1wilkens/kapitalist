use chrono::{NaiveDateTime};

use request::*;
use schema::users;
use util;

#[derive(Debug, Deserialize, Serialize)]
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub secret_hash: String,
    pub username: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
#[derive(Insertable)]
#[table_name ="users"]
pub struct NewUser {
    pub email: String,
    pub secret_hash: String,
    pub username: String,
}

impl NewUser {
    pub fn from_request(req: UserCreationRequest) -> Result<NewUser, ::std::io::Error> {
        use pwhash::scrypt::scrypt_simple;

        let params = util::get_scrypt_params();
        // Unwrap is safe here because scrypt_simple does not ever return an error
        let hash = scrypt_simple(&req.password, &params).unwrap();
        Ok(NewUser { email: req.email, secret_hash: hash, username: req.name })
    }
}