use actix_web::{
    actix::{Handler, Message},
    error::{self, Error},
};
use chrono::{NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
use slog::trace;

use kapitalist_types::request::{TransactionCreationRequest, TransactionUpdateRequest};

use crate::db::{schema::transactions, wallet::GetWallet, DatabaseExecutor};

/// Database entity representing a transaction
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable, AsChangeset)]
pub struct Transaction {
    pub name: String,
    pub id: i64,
    pub wallet_id: i64,
    pub category_id: i64,
    pub amount: i64,
    pub ts: NaiveDateTime,
}

/// Insertable database entity to create new transactions
#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub name: String,
    pub wallet_id: i64,
    pub category_id: i64,
    pub amount: i64,
    pub ts: NaiveDateTime,
}

/// Activ message to create a new transaction in the the database
#[derive(Debug)]
pub struct CreateNewTransaction {
    user_id: i64,
    tx: NewTransaction,
}

/// Actix message to retrieve a transaction entity from the database
#[derive(Debug)]
pub struct GetTransaction {
    pub(crate) uid: i64,
    pub(crate) tid: i64,
}

/// Actix message to retrieve all transactions of a given wallet from the database
#[derive(Debug)]
pub struct GetTransactionsFromWallet {
    pub(crate) uid: i64,
    pub(crate) wid: i64,
}

#[derive(Debug)]
pub struct UpdateTransaction {
    pub(crate) uid: i64,
    pub(crate) tid: i64,
    pub(crate) name: Option<String>,
    pub(crate) wallet_id: Option<i64>,
    pub(crate) category_id: Option<i64>,
    pub(crate) amount: Option<i64>,
    pub(crate) ts: Option<NaiveDateTime>,
}

/// Actix message to delete a transaction entity from the database
#[derive(Debug)]
pub struct DeleteTransaction {
    pub(crate) uid: i64,
    pub(crate) tid: i64,
}

impl CreateNewTransaction {
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_request(user_id: i64, req: TransactionCreationRequest) -> Self {
        Self {
            user_id: user_id,
            tx: NewTransaction {
                name: req.name,
                wallet_id: req.wallet_id,
                category_id: req.category_id,
                amount: req.amount,
                ts: req
                    .ts
                    // XXX: Check this is correct in regards to timezone
                    .unwrap_or_else(|| NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
            },
        }
    }
}

impl Message for CreateNewTransaction {
    type Result = Result<Option<Transaction>, Error>;
}

impl Handler<CreateNewTransaction> for DatabaseExecutor {
    type Result = Result<Option<Transaction>, Error>;

    fn handle(&mut self, msg: CreateNewTransaction, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::transactions::dsl::*;
        trace!(self.1, "Received db action"; "msg" => ?msg);

        let wallet = self.handle(GetWallet::new(msg.user_id, msg.tx.wallet_id), ctx);
        let result = match wallet {
            Ok(Some(_)) => {
                // User owns the target wallet
                let tx = diesel::insert_into(transactions)
                    .values(&msg.tx)
                    .get_result(&self.0)
                    .map_err(error::ErrorInternalServerError)?;
                Some(tx)
            }
            _ => None,
        };

        trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl GetTransaction {
    pub fn new(user_id: i64, transaction_id: i64) -> Self {
        Self {
            uid: user_id,
            tid: transaction_id,
        }
    }
}

impl Message for GetTransaction {
    type Result = Result<Option<Transaction>, Error>;
}

impl Handler<GetTransaction> for DatabaseExecutor {
    type Result = Result<Option<Transaction>, Error>;

    #[allow(clippy::single_match_else)]
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
    pub fn new(user_id: i64, wallet_id: i64) -> Self {
        Self {
            uid: user_id,
            wid: wallet_id,
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

impl UpdateTransaction {
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_request(user_id: i64, transaction_id: i64, req: TransactionUpdateRequest) -> Self {
        Self {
            uid: user_id,
            tid: transaction_id,
            name: req.name,
            wallet_id: req.wallet_id,
            category_id: req.category_id,
            amount: req.amount,
            ts: req.ts,
        }
    }
}

impl Message for UpdateTransaction {
    type Result = Result<Option<Transaction>, Error>;
}

impl Handler<UpdateTransaction> for DatabaseExecutor {
    type Result = Result<Option<Transaction>, Error>;

    fn handle(&mut self, msg: UpdateTransaction, ctx: &mut Self::Context) -> Self::Result {
        trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let transaction = self.handle(GetTransaction::new(msg.uid, msg.tid), ctx);
        let result = match transaction {
            Ok(Some(mut tx)) => {
                if let Some(wallet_id) = msg.wallet_id {
                    tx.wallet_id = wallet_id;
                }
                if let Some(category_id) = msg.category_id {
                    tx.category_id = category_id;
                }
                if let Some(amount) = msg.amount {
                    tx.amount = amount;
                }
                if let Some(ts) = msg.ts {
                    tx.ts = ts;
                }
                diesel::update(&tx)
                    .set(&tx)
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

impl DeleteTransaction {
    pub fn new(user_id: i64, transaction_id: i64) -> Self {
        Self {
            uid: user_id,
            tid: transaction_id,
        }
    }
}

impl Message for DeleteTransaction {
    type Result = Result<bool, Error>;
}

impl Handler<DeleteTransaction> for DatabaseExecutor {
    type Result = Result<bool, Error>;

    fn handle(&mut self, msg: DeleteTransaction, ctx: &mut Self::Context) -> Self::Result {
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
