use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
//use slog::trace;

use kapitalist_types::request::{WalletCreationRequest, WalletUpdateRequest};
use kapitalist_types::response::WalletResponse;

use crate::db::schema::wallets;

// XXX: Find out how to handle logging here

// XXX: Make wallet_type an enum once we figure out which values belong there
/// Database entity representing a user's wallet
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable, AsChangeset)]
pub struct Wallet {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub wallet_type: String,
    pub initial_balance: i64,
    pub current_balance: i64,
    pub color: String,
    pub created_at: NaiveDateTime,
}

/// Insertable database entity to create new wallets
#[derive(Debug, Insertable)]
#[table_name = "wallets"]
pub struct NewWallet {
    pub user_id: i64,
    pub name: String,
    pub wallet_type: String,
    pub initial_balance: i64,
    pub current_balance: i64,
    pub color: String,
}

/// Actix message to retrieve a wallet entity from the database
#[derive(Debug)]
pub struct GetWallet {
    pub(crate) uid: i64,
    pub(crate) wid: i64,
}

/// Actix message to retrieve all wallet entities of a given user from the database
#[derive(Debug)]
pub struct GetWalletsFromUser {
    pub(crate) uid: i64,
}

/// Actix message to update a wallet entity in the database
#[derive(Debug)]
pub struct UpdateWallet {
    pub uid: i64,
    pub wid: i64,
    pub name: Option<String>,
    pub wallet_type: Option<String>,
    pub color: Option<String>,
}

/// Actix message to delete a wallet entity from the database
#[derive(Debug)]
pub struct DeleteWallet {
    pub(crate) uid: i64,
    pub(crate) wid: i64,
}

impl Wallet {
    pub fn into_response(self) -> WalletResponse {
        WalletResponse {
            id: self.id,
            name: self.name,
            wallet_type: self.wallet_type,
            current_balance: self.current_balance,
            color: self.color,
            created_at: self.created_at,
        }
    }
}

impl NewWallet {
    pub fn from_request(uid: i64, req: WalletCreationRequest) -> Self {
        // Color is randomly chosen if unset
        let color = if let Some(col) = req.color {
            col
        } else {
            // XXX: Always Zelda green for now
            // Replace with random harmonic color similar to:
            // https://stackoverflow.com/a/43235
            "#088C5A".to_string()
        };
        // Balance defaults to 0 if unset
        let balance = req.balance.unwrap_or(0);

        Self {
            user_id: uid,
            name: req.name,
            wallet_type: req.wallet_type,
            initial_balance: balance,
            current_balance: balance,
            color: color,
        }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<Wallet, &'static str> {
        use crate::db::schema::wallets::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        let wallet = diesel::insert_into(wallets)
            .values(self)
            .get_result(conn)
            .map_err(|_| "Error inserting new Wallet into database")?;
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?wallet);
        Ok(wallet)
    }
}

impl GetWallet {
    pub fn new(user_id: i64, wallet_id: i64) -> Self {
        Self {
            wid: wallet_id,
            uid: user_id,
        }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<Result<Wallet, ()>, &'static str> {
        use crate::db::schema::wallets::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let wallet = wallets
            .filter(id.eq(&self.wid))
            .filter(user_id.eq(&self.uid))
            .get_result(conn)
            .optional()
            .map_err(|_| "Error getting Wallet from database")?
            .ok_or(());
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?wallet);
        Ok(wallet)
    }
}

impl GetWalletsFromUser {
    pub fn new(user_id: i64) -> Self {
        Self { uid: user_id }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<Option<Vec<Wallet>>, &'static str> {
        use crate::db::schema::wallets::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        let result = wallets
            .filter(user_id.eq(self.uid))
            .get_results(conn)
            .optional()
            .map_err(|_| "Error getting Wallets from database")?;
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl UpdateWallet {
    pub fn from_request(user_id: i64, wallet_id: i64, req: WalletUpdateRequest) -> Self {
        Self {
            uid: user_id,
            wid: wallet_id,
            name: req.name,
            wallet_type: req.wallet_type,
            color: req.color,
        }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<Option<Wallet>, &'static str> {
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let wallet = GetWallet::new(self.uid, self.wid).execute(conn);
        let result = match wallet {
            Ok(Ok(mut w)) => {
                if let Some(ref name) = self.name {
                    w.name = name.clone();
                }
                if let Some(ref wallet_type) = self.wallet_type {
                    w.wallet_type = wallet_type.clone();
                }
                if let Some(ref color) = self.color {
                    w.color = color.clone()
                }
                diesel::update(&w)
                    .set(&w)
                    .get_result(conn)
                    .optional()
                    .map_err(|_| "Error updating Wallet in database")?
            }
            _ => None,
        };
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl DeleteWallet {
    pub fn new(user_id: i64, wallet_id: i64) -> Self {
        Self {
            uid: user_id,
            wid: wallet_id,
        }
    }

    pub fn execute(self, conn: &PgConnection) -> Result<bool, &'static str> {
        use crate::db::schema::wallets::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let res = diesel::delete(wallets)
            .filter(id.eq(&self.wid))
            .filter(user_id.eq(&self.uid))
            .execute(conn)
            .map_err(|_| "Error deleting Wallet from database")?;
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?res);
        Ok(res > 0)
    }
}
