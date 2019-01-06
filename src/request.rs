use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserCreationRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserUpdateRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct WalletCreationRequest {
    pub name: String,
    pub wallet_type: String,
    pub balance: i32,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct CategoryCreationRequest {
    pub name: String,
    pub parent_id: Option<i32>,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionCreationRequest {
    pub source_wallet_id: i32,
    pub destination_wallet_id: Option<i32>,
    pub category_id: i32,
    pub amount: i32,
    pub ts: NaiveDateTime,
}
