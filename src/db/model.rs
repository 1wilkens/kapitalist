use chrono::NaiveDateTime;

use super::schema::{users, wallets};
use request::*;
use util;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Wallet {
    pub id: i32,
    pub name: String,
    pub initial_balance: i32,
    pub current_balance: i32,
    pub color: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "wallets"]
pub struct NewWallet {
    pub name: String,
    pub initial_balance: i32,
    pub current_balance: i32,
    pub color: String,
}

impl NewWallet {
    pub fn from_request(req: WalletCreationRequest) -> NewWallet {
        NewWallet {
            name: req.name,
            initial_balance: req.balance,
            current_balance: req.balance,
            color: req.color,
        }
    }
}
