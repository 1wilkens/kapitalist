use chrono::{NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};
//use slog::trace;

use kapitalist_types::request::{TransactionCreationRequest, TransactionUpdateRequest};
use kapitalist_types::response::TransactionResponse;

use crate::db::{schema::transactions, wallet::GetWallet};

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

/// Struct to retrieve a transaction entity from the database
#[derive(Debug)]
pub struct GetTransaction {
    pub uid: i64,
    pub tid: i64,
}

/// Struct to retrieve all transactions of a given wallet from the database
#[derive(Debug)]
pub struct GetTransactionsFromWallet {
    pub uid: i64,
    pub wid: i64,
}

#[derive(Debug)]
pub struct UpdateTransaction {
    pub uid: i64,
    pub tid: i64,
    pub name: Option<String>,
    pub wallet_id: Option<i64>,
    pub category_id: Option<i64>,
    pub amount: Option<i64>,
    pub ts: Option<NaiveDateTime>,
}

/// Struct to delete a transaction entity from the database
#[derive(Debug)]
pub struct DeleteTransaction {
    pub uid: i64,
    pub tid: i64,
}

impl Transaction {
    pub fn into_response(self) -> TransactionResponse {
        TransactionResponse {
            id: self.id,
            name: self.name,
            wallet_id: self.wallet_id,
            category_id: self.category_id,
            amount: self.amount,
            ts: self.ts,
        }
    }
}

impl CreateNewTransaction {
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

    pub fn execute(self, conn: &PgConnection) -> Result<Option<Transaction>, &'static str> {
        use crate::db::schema::transactions::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        let wallet = GetWallet::new(self.user_id, self.tx.wallet_id).execute(conn);
        let result = match wallet {
            Ok(Ok(_)) => {
                // User owns the target wallet
                let tx = diesel::insert_into(transactions)
                    .values(self.tx)
                    .get_result(conn)
                    .map_err(|_| "Error inserting Transaction into database")?;
                Some(tx)
            }
            _ => None,
        };

        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
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

    #[allow(clippy::single_match_else)]
    pub fn execute(self, conn: &PgConnection) -> Result<Option<Transaction>, &'static str> {
        use crate::db::schema::transactions::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        let transaction: Option<Transaction> = transactions
            .filter(id.eq(self.tid))
            .get_result(conn)
            .optional()
            .map_err(|_| "Error getting Transaction from database")?;

        let transaction = match transaction {
            Some(t) => t,
            None => {
                // XXX: This is ugly
                //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => "Ok(None(");
                return Ok(None);
            }
        };

        // XXX: Verify this is enough to protect against unauthorized access
        let wallet = GetWallet::new(transaction.wallet_id, self.uid).execute(conn);
        let result = match wallet {
            Ok(Ok(_)) => Some(transaction),
            _ => None,
        };

        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
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

    pub fn execute(self, conn: &PgConnection) -> Result<Option<Vec<Transaction>>, &'static str> {
        use crate::db::schema::transactions::dsl::*;
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        // Check user has access to source wallet
        let wallet = GetWallet::new(self.wid, self.uid).execute(conn);
        let result = match wallet {
            // XXX: verify this is correct
            // User owns the wallet
            Ok(Ok(_)) => {
                let txs = transactions
                    .filter(wallet_id.eq(self.wid))
                    .get_results(conn)
                    .optional()
                    .map_err(|_| "Error getting Transactions from database")?;
                txs
            }
            // User doesn't own the wallet or it doesn't exist (yet)
            _ => None,
        };

        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}

impl UpdateTransaction {
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

    pub fn execute(self, conn: &PgConnection) -> Result<Option<Transaction>, &'static str> {
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        // XXX: Verify this is enough to protect unauthorized access
        let transaction = GetTransaction::new(self.uid, self.tid).execute(conn);
        let result = match transaction {
            Ok(Some(mut tx)) => {
                if let Some(wallet_id) = self.wallet_id {
                    tx.wallet_id = wallet_id;
                }
                if let Some(category_id) = self.category_id {
                    tx.category_id = category_id;
                }
                if let Some(amount) = self.amount {
                    tx.amount = amount;
                }
                if let Some(ts) = self.ts {
                    tx.ts = ts;
                }
                diesel::update(&tx)
                    .set(&tx)
                    .get_result(conn)
                    .optional()
                    .map_err(|_| "Error updating Transaction in database")?
            }
            _ => None,
        };
        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
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

    pub fn execute(self, conn: &PgConnection) -> Result<bool, &'static str> {
        //trace!(self.1, "Received db action"; "msg" => ?msg);

        let tx = match GetTransaction::new(self.tid, self.uid).execute(conn) {
            Ok(Some(t)) => t,
            _ => return Ok(false),
        };

        // XXX: Verify this is enough to protect against unauthorized access
        let result = match GetWallet::new(tx.wallet_id, self.uid).execute(conn) {
            // User owns the Wallet and is thus able to delete the Transaction
            Ok(Ok(_)) => {
                diesel::delete(&tx)
                    .execute(conn)
                    .map_err(|_| "Error deleting Transaction from database")?
                    > 0
            }
            _ => false,
        };

        //trace!(self.1, "Handled db action"; "msg" => ?msg, "result" => ?result);
        Ok(result)
    }
}
