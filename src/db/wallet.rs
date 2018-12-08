use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};

use crate::db::{schema::wallets, DatabaseExecutor};
use crate::request::WalletCreationRequest;

/// Database entity representing a user's wallet
///
/// id              -
/// user_id         -
/// name            -
/// initial_balance -
/// current_balance -
/// color           -
/// created_at      -
#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub initial_balance: i32,
    pub current_balance: i32,
    pub color: Option<String>,
    pub created_at: NaiveDateTime,
}

/// Insertable database entity to create new wallets
///
/// user_id         -
/// name            -
/// initial_balance -
/// current_balance -
/// color           -
#[derive(Debug, Insertable)]
#[table_name = "wallets"]
pub struct NewWallet {
    pub user_id: i32,
    pub name: String,
    pub initial_balance: i32,
    pub current_balance: i32,
    pub color: String,
}

/// Actix message to retrieve a wallet entity from the database
#[derive(Debug)]
pub struct GetWallet {
    pub(crate) wid: i32,
    pub(crate) uid: i32,
}

impl NewWallet {
    pub fn from_request(req: WalletCreationRequest, uid: i32) -> NewWallet {
        NewWallet {
            name: req.name,
            user_id: uid,
            initial_balance: req.balance,
            current_balance: req.balance,
            color: req.color,
        }
    }
}

impl Message for NewWallet {
    type Result = Result<Wallet, Error>;
}

impl Handler<NewWallet> for DatabaseExecutor {
    type Result = Result<Wallet, Error>;

    fn handle(&mut self, msg: NewWallet, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::wallets::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Figure out error type to be used here and add conversion functions for convenience
        /*let exists: bool = diesel::select(diesel::dsl::exists(wallets.filter(email.eq(&msg.email))))
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error getting User from Db"))?;

        if exists {
            // TODO: should we really return this message?
            return Err(error::ErrorUnauthorized("User already exists"));
        }*/

        let wallet: Wallet = diesel::insert_into(wallets)
            .values(&msg)
            .get_result(&self.0)
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?wallet);
        Ok(wallet)
    }
}

impl GetWallet {
    pub fn new(wallet_id: i32, user_id: i32) -> GetWallet {
        GetWallet {
            wid: wallet_id,
            uid: user_id,
        }
    }
}

impl Message for GetWallet {
    type Result = Result<Option<Wallet>, Error>;
}

impl Handler<GetWallet> for DatabaseExecutor {
    type Result = Result<Option<Wallet>, Error>;

    fn handle(&mut self, msg: GetWallet, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::wallets::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let wallet = wallets
            .filter(id.eq(&msg.wid))
            .filter(user_id.eq(&msg.uid))
            .get_result(&self.0)
            .optional()
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?wallet);
        Ok(wallet)
    }
}
