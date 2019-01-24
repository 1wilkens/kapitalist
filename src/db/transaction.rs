use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::{NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use slog::trace;

use crate::db::{schema::transactions, wallet::GetWallet, DatabaseExecutor};
use crate::request::TransactionCreationRequest;

/// Database entity representing a transaction
///
/// id          -
/// wallet_id   -
/// category_id -
/// amount      -
/// ts          -
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable, AsChangeset)]
pub struct Transaction {
    pub id: i64,
    pub wallet_id: i64,
    pub category_id: i64,
    pub amount: i64,
    pub ts: NaiveDateTime,
}

/// Insertable database entity to create new transactions
///
/// wallet_id   -
/// category_id -
/// amount      -
/// ts          -
#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub wallet_id: i64,
    pub category_id: i64,
    pub amount: i64,
    pub ts: NaiveDateTime,
}

/// Actix message to retrieve a transaction entity from the database
#[derive(Debug)]
pub struct GetTransaction {
    pub(crate) tid: i64,
    pub(crate) uid: i64,
}

/// Actix message to retrieve all transactions of a given wallet from the database
#[derive(Debug)]
pub struct GetTransactionsFromWallet {
    pub(crate) wid: i64,
    pub(crate) uid: i64,
}

/// Actix message to delete a transaction entity from the database
#[derive(Debug)]
pub struct DeleteTransaction {
    pub(crate) tid: i64,
    pub(crate) uid: i64,
}

impl NewTransaction {
    pub fn from_request(req: TransactionCreationRequest) -> NewTransaction {
        NewTransaction {
            wallet_id: req.wallet_id,
            category_id: req.category_id,
            amount: req.amount,
            ts: req
                .ts
                // XXX: Check this is correct in regards to timezone
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
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

        // XXX: This currently does NOT check if the user owns the source wallet
        // Unfortunately we can't just add a user_id field to NewTransaction as it is directly
        // Insertable. TODO: Figure out an elegant way to handle this!
        let transaction = diesel::insert_into(transactions)
            .values(&msg)
            .get_result(&self.0)
            .map_err(error::ErrorInternalServerError)?;
        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?transaction);
        Ok(transaction)
    }
}

impl GetTransaction {
    pub fn new(transaction_id: i64, user_id: i64) -> GetTransaction {
        GetTransaction {
            tid: transaction_id,
            uid: user_id,
        }
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
            None => {
                // XXX: This is ugly
                trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => "Ok(None(");
                return Ok(None);
            }
        };

        // XXX: Verify this is enough to protect against unauthorized access
        let wallet = self.handle(GetWallet::new(transaction.wallet_id, msg.uid), ctx);
        let result = match wallet {
            Ok(Some(_)) => Some(transaction),
            _ => None,
        };

        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl GetTransactionsFromWallet {
    pub fn new(wallet_id: i64, user_id: i64) -> GetTransactionsFromWallet {
        GetTransactionsFromWallet {
            wid: wallet_id,
            uid: user_id,
        }
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

        // Check user has access to source wallet
        let wallet = self.handle(GetWallet::new(msg.wid, msg.uid), ctx);
        let result = match wallet {
            Ok(Some(_)) => {
                let txs = transactions
                    .filter(wallet_id.eq(msg.wid))
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

impl DeleteTransaction {
    pub fn new(transaction_id: i64, user_id: i64) -> DeleteTransaction {
        DeleteTransaction {
            tid: transaction_id,
            uid: user_id,
        }
    }
}

impl Message for DeleteTransaction {
    type Result = Result<bool, Error>;
}

impl Handler<DeleteTransaction> for DatabaseExecutor {
    type Result = Result<bool, Error>;

    fn handle(&mut self, msg: DeleteTransaction, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::transactions::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        let tx = self.handle(GetTransaction::new(msg.tid, msg.uid), ctx);
        let tx = match tx {
            Ok(Some(t)) => t,
            _ => return Ok(false),
        };

        // XXX: Verify this is enough to protect against unauthorized access
        let wallet = self.handle(GetWallet::new(tx.wallet_id, msg.uid), ctx);
        let result = match wallet {
            Ok(Some(_)) => diesel::delete(&tx)
                .execute(&self.0)
                .map_err(error::ErrorInternalServerError)?,
            _ => 0,
        };

        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result > 0)
    }
}
