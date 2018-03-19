use chrono::NaiveDateTime;

use schema::users;

#[derive(Debug, Serialize)]
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub secret_hash: String,
    pub username: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name ="users"]
pub struct NewUser {
    pub email: String,
    pub secret_hash: String,
    pub username: String,
}

impl NewUser {
    pub fn from_request(req: UserCreationRequest) -> Result<NewUser, ::std::io::Error> {
        let params = ::pwhash::scrypt::ScryptParams::new(15, 8, 1);
        let hash = ::pwhash::scrypt::scrypt_simple(&req.password, &params)?;
        Ok(NewUser { email: req.email, secret_hash: hash, username: req.name })
    }
}

#[derive(Debug, Deserialize)]
pub struct UserCreationRequest {
    pub email:    String,
    pub password: String,
    pub name:     String,
}

#[derive(Debug, Deserialize)]
pub struct UserUpdateRequest {
    pub email:    Option<String>,
    pub password: Option<String>,
    pub name:     Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub email:    String,
    pub password: String,
}

