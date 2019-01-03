use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use slog::trace;

use crate::db::{
    schema::transactions,
    wallet::{GetWallet, Wallet},
    DatabaseExecutor,
};
use crate::request::TransactionCreationRequest;

/// Database entity representing a transaction
///
/// id                    -
/// source_wallet_id      -
/// destination_wallet_id -
/// category_id           -
/// amount                -
/// ts                    -
#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Transaction {
    pub id: i32,
    pub source_wallet_id: i32,
    pub destination_wallet_id: Option<i32>,
    pub category_id: i32,
    pub amount: i32,
    pub ts: NaiveDateTime,
}

/// Insertable database entity to create new transactions
///
/// source_wallet_id      -
/// destination_wallet_id -
/// category_id           -
/// amount                -
/// ts                    -
#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub source_wallet_id: i32,
    pub destination_wallet_id: Option<i32>,
    pub category_id: i32,
    pub amount: i32,
    pub ts: NaiveDateTime,
}

/// Actix message to retrieve a transaction entity from the database
#[derive(Debug)]
pub struct GetTransaction {
    pub(crate) tid: i32,
    pub(crate) uid: i32,
}

/// Actix message to retrieve all transactions of a given wallet from the database
#[derive(Debug)]
pub struct GetTransactionsFromWallet {
    pub(crate) wid: i32,
    pub(crate) uid: i32,
}

impl NewTransaction {
    pub fn from_request(req: TransactionCreationRequest) -> NewTransaction {
        NewTransaction {
            source_wallet_id: req.source_wallet_id,
            destination_wallet_id: req.destination_wallet_id,
            category_id: req.category_id,
            amount: req.amount,
            ts: req.ts,
        }
    }
}

impl Message for NewTransaction {
    type Result = Result<Transaction, Error>;
}

impl Handler<NewTransaction> for DatabaseExecutor {
    type Result = Result<Transaction, Error>;

    fn handle(&mut self, msg: NewTransaction, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::transactions::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Figure out error type to be used here and add conversion functions for convenience
        /*let exists: bool = diesel::select(diesel::dsl::exists(wallets.filter(email.eq(&msg.email))))
            .get_result(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Error getting User from Db"))?;

        if exists {
            // TODO: should we really return this message?
            return Err(error::ErrorUnauthorized("User already exists"));
        }*/

        let transaction: Transaction = diesel::insert_into(transactions)
            .values(&msg)
            .get_result(&self.0)
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?transaction);
        Ok(transaction)
    }
}

impl Message for GetTransaction {
    type Result = Result<Option<Transaction>, Error>;
}

impl Handler<GetTransaction> for DatabaseExecutor {
    type Result = Result<Option<Transaction>, Error>;

    fn handle(&mut self, msg: GetTransaction, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::transactions::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        let transaction: Option<Transaction> = transactions
            .filter(id.eq(&msg.tid))
            .get_result(&self.0)
            .optional()
            .map_err(error::ErrorInternalServerError)?;

        let transaction = match transaction {
            Some(t) => t,
            None => return Ok(None),
        };

        // XXX: Verify this is enough to protect against unauthorized access
        let wallet = self.handle(GetWallet::new(transaction.source_wallet_id, msg.uid), ctx);

        let result = match wallet {
            Ok(Some(_)) => Some(transaction),
            _ => None,
        };

        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl Message for GetTransactionsFromWallet {
    type Result = Result<Option<Vec<Transaction>>, Error>;
}

impl Handler<GetTransactionsFromWallet> for DatabaseExecutor {
    type Result = Result<Option<Vec<Transaction>>, Error>;

    fn handle(&mut self, msg: GetTransactionsFromWallet, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::transactions::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        let get_wallet = GetWallet::new(msg.wid, msg.uid);
        let wallet = self.handle(get_wallet, ctx);

        let result = match wallet {
            Ok(Some(_)) => {
                let txs = transactions
                    .filter(source_wallet_id.eq(msg.wid))
                    .get_results(&self.0)
                    .map_err(error::ErrorInternalServerError)?;
                Some(txs)
            }
            _ => None,
        };

        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}
