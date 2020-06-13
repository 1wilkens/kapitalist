/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Result | Description |
/// | :--: | -- | -- | -- | -- |
/// | POST | `/transaction` | `TransactionCreationRequest` | create new transaction |
/// | GET | `/transaction/{tid}` | - | get transaction details |
/// | PUT | `/transaction/{tid}` | `TransactionUpdateRequest` | update transaction details |
/// | DELETE | `/transaction/{tid}` | - | delete transaction |
/// | GET | `/transactions` | `from, to` | get transaction history |
///
use tracing::debug;
use warp::{reject, reply, Rejection, Reply};

use kapitalist_types::request::{TransactionCreationRequest, TransactionUpdateRequest};
use kapitalist_types::response::TransactionResponse;

//use crate::api::util::{reject::reject, reject::reject, update_request_invalid};
use crate::auth::User;
use crate::db::{
    transaction::{
        CreateNewTransaction, DeleteTransaction, GetTransaction, GetTransactionsFromWallet,
        Transaction, UpdateTransaction,
    },
    Database,
};

pub async fn post(
    db: Database,
    user: User,
    req: TransactionCreationRequest,
) -> Result<impl Reply, Rejection> {
    let new_tx = CreateNewTransaction::from_request(user.user_id, req);
    match new_tx.execute(&*db.0) {
        Ok(Some(tx)) => {
            let url = format!("/transaction/{}", tx.id);
            //Ok(status::Created(url, Some(reply::json(tx.into_response()))))
            Ok(reply::json(&tx.into_response()))
        }
        //Ok(None) => Err(reject::reject("transaction")),
        Ok(None) => Err(reject::reject()),
        Err(err) => {
            debug!(error = %&err, "Error inserting transaction into database");
            Err(reject::reject())
        }
    }
}

pub async fn get(db: Database, user: User, tid: i64) -> Result<impl Reply, Rejection> {
    let get_tx = GetTransaction::new(user.user_id, tid);
    match get_tx.execute(&*db.0) {
        Ok(Some(tx)) => Ok(reply::json(&tx.into_response())),
        //Ok(None) => Err(reject::reject("transaction")),
        Ok(None) => Err(reject::reject()),
        Err(err) => {
            debug!(error = %&err, "Error getting transaction from database");
            Err(reject::reject())
        }
    }
}

pub async fn put(
    db: Database,
    user: User,
    tid: i64,
    req: TransactionUpdateRequest,
) -> Result<impl Reply, Rejection> {
    if !req.is_valid() {
        return Err(reject::reject());
    }

    let update_tx = UpdateTransaction::from_request(user.user_id, tid, req);
    match update_tx.execute(&*db.0) {
        Ok(Some(tx)) => Ok(reply::json(&tx.into_response())),
        //Ok(None) => Err(reject::reject(&"transaction"))
        Ok(None) => Err(reject::reject()),
        Err(err) => {
            debug!(error = %&err, "Error updating transaction in database");
            Err(reject::reject())
        }
    }
}

pub async fn delete(db: Database, user: User, tid: i64) -> Result<impl Reply, Rejection> {
    let delete_tx = DeleteTransaction::new(tid, user.user_id);
    match delete_tx.execute(&*db.0) {
        Ok(true) => Ok(reply::json(&())),
        //Ok(false) => Err(reject::reject("transaction")),
        Ok(false) => Err(reject::reject()),
        Err(err) => {
            debug!(error = %&err, "Error deleting transaction from database");
            Err(reject::reject())
        }
    }
}

pub async fn all(db: Database, user: User, wid: i64) -> Result<impl Reply, Rejection> {
    let get_txs = GetTransactionsFromWallet::new(user.user_id, wid);
    let resp: Vec<TransactionResponse> = match get_txs.execute(&*db.0) {
        Ok(Some(txs)) => txs.into_iter().map(Transaction::into_response).collect(),
        Ok(None) => vec![], // Wallet has no Transactions yet
        Err(err) => {
            debug!(error = %&err, "Error getting transactions from database");
            return Err(reject::reject());
        }
    };
    Ok(reply::json(&resp))
}
