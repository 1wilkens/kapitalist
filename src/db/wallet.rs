use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use slog::trace;

use crate::db::{schema::wallets, DatabaseExecutor};
use crate::request::{WalletCreationRequest, WalletUpdateRequest};

// XXX: Make wallet_type an enum once we figure out which values belong there

/// Database entity representing a user's wallet
///
/// id              -
/// user_id         -
/// name            -
/// wallet_type     -
/// initial_balance -
/// current_balance -
/// color           -
/// created_at      -
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable, AsChangeset)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub wallet_type: String,
    pub initial_balance: i32,
    pub current_balance: i32,
    pub color: String,
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
    pub wallet_type: String,
    pub initial_balance: i32,
    pub current_balance: i32,
    pub color: String,
}

/// Actix message to retrieve a wallet entity from the database
#[derive(Debug)]
pub struct GetWallet {
    pub(crate) uid: i32,
    pub(crate) wid: i32,
}

/// Actix message to update a wallet entity in the database
#[derive(Debug)]
pub struct UpdateWallet {
    pub uid: i32,
    pub wid: i32,
    pub name: Option<String>,
    pub wallet_type: Option<String>,
    pub color: Option<String>,
}

/// Actix message to delete a wallet entity from the database
#[derive(Debug)]
pub struct DeleteWallet {
    pub(crate) uid: i32,
    pub(crate) wid: i32,
}

impl NewWallet {
    pub fn from_request(uid: i32, req: WalletCreationRequest) -> NewWallet {
        NewWallet {
            user_id: uid,
            name: req.name,
            wallet_type: req.wallet_type,
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

        let wallet: Wallet = diesel::insert_into(wallets)
            .values(&msg)
            .get_result(&self.0)
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?wallet);
        Ok(wallet)
    }
}

impl GetWallet {
    pub fn new(user_id: i32, wallet_id: i32) -> GetWallet {
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

impl UpdateWallet {
    pub fn from_request(uid: i32, wid: i32, req: WalletUpdateRequest) -> UpdateWallet {
        UpdateWallet {
            uid: uid,
            wid: wid,
            name: req.name,
            wallet_type: req.wallet_type,
            color: req.color,
        }
    }
}

impl Message for UpdateWallet {
    type Result = Result<Option<Wallet>, Error>;
}

impl Handler<UpdateWallet> for DatabaseExecutor {
    type Result = Result<Option<Wallet>, Error>;

    fn handle(&mut self, msg: UpdateWallet, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::wallets::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let wallet = self.handle(GetWallet::new(msg.uid, msg.wid), ctx);
        let result = match wallet {
            Ok(Some(mut w)) => {
                if let Some(ref n) = msg.name {
                    w.name = n.clone();
                }
                if let Some(ref wt) = msg.wallet_type {
                    w.wallet_type = wt.clone();
                }
                if let Some(ref c) = msg.color {
                    w.color = c.clone()
                }
                diesel::update(&w)
                    .set(&w)
                    .get_result(&self.0)
                    .optional()
                    .map_err(error::ErrorInternalServerError)?
            }
            _ => None,
        };
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl DeleteWallet {
    pub fn new(user_id: i32, wallet_id: i32) -> DeleteWallet {
        DeleteWallet {
            wid: wallet_id,
            uid: user_id,
        }
    }
}

impl Message for DeleteWallet {
    type Result = Result<bool, Error>;
}

impl Handler<DeleteWallet> for DatabaseExecutor {
    type Result = Result<bool, Error>;

    fn handle(&mut self, msg: DeleteWallet, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::wallets::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let res = diesel::delete(wallets)
            .filter(id.eq(&msg.wid))
            .filter(user_id.eq(&msg.uid))
            .execute(&self.0)
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?res);
        Ok(res > 0)
    }
}
