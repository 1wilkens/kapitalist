/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Result | Description |
/// | :--: | -- | -- | -- | -- |
/// | GET | `/transactions` | `from, to` | get transaction history |
/// | POST | `/transaction` | `TransactionCreationRequest` | create new transaction |
/// | GET | `/transaction/{tid}` | - | get transaction details |
/// | PUT | `/transaction/{tid}` | `TransactionUpdateRequest` | update transaction details |
/// | DELETE | `/transaction/{tid}` | - | delete transaction |
///
use rocket::{response::status, State};
use rocket_contrib::json::Json;
use slog::debug;

use kapitalist_types::request::{TransactionCreationRequest, TransactionUpdateRequest};
use kapitalist_types::response::TransactionResponse;

use crate::api::util::{internal_server_error, not_found, update_request_invalid};
use crate::auth::User;
use crate::db::{
    transaction::{
        CreateNewTransaction, DeleteTransaction, GetTransaction, GetTransactionsFromWallet, Transaction,
        UpdateTransaction,
    },
    Database,
};
use crate::state::AppState;

#[post("/", data = "<req>")]
pub fn post(
    user: User,
    state: State<AppState>,
    db: Database,
    req: Json<TransactionCreationRequest>,
) -> super::Result<status::Created<Json<TransactionResponse>>> {
    let new_tx = CreateNewTransaction::from_request(user.user_id, req.0);
    match new_tx.execute(&*db) {
        Ok(Some(tx)) => {
            let url = format!("/transaction/{}", tx.id);
            Ok(status::Created(url, Some(Json(tx.into_response()))))
        }
        Ok(None) => Err(not_found("transaction")),
        Err(err) => {
            debug!(&state.log, "Error inserting transaction into database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[get("/all/<wid>")]
pub fn get_all(
    user: User,
    state: State<AppState>,
    db: Database,
    wid: i64,
) -> super::Result<Json<Vec<TransactionResponse>>> {
    let get_txs = GetTransactionsFromWallet::new(user.user_id, wid);
    match get_txs.execute(&*db) {
        Ok(Some(txs)) => Ok(Json(txs.into_iter().map(Transaction::into_response).collect())),
        Ok(None) => Ok(Json(Vec::new())), // Wallet has no Transactions yet
        Err(err) => {
            debug!(&state.log, "Error getting transactions from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[get("/<tid>")]
pub fn get(user: User, state: State<AppState>, db: Database, tid: i64) -> super::Result<Json<TransactionResponse>> {
    let get_tx = GetTransaction::new(user.user_id, tid);
    match get_tx.execute(&*db) {
        Ok(Some(tx)) => Ok(Json(tx.into_response())),
        Ok(None) => Err(not_found("transaction")),
        Err(err) => {
            debug!(&state.log, "Error getting transaction from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[put("/<tid>", data = "<req>")]
pub fn put(
    user: User,
    state: State<AppState>,
    db: Database,
    tid: i64,
    req: Json<TransactionUpdateRequest>,
) -> super::Result<Json<TransactionResponse>> {
    if !req.is_valid() {
        return Err(update_request_invalid());
    }

    let update_tx = UpdateTransaction::from_request(user.user_id, tid, req.0);
    match update_tx.execute(&*db) {
        Ok(Some(tx)) => Ok(Json(tx.into_response())),
        Ok(None) => Err(not_found(&"transaction")),
        Err(err) => {
            debug!(&state.log, "Error updating transaction in database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[delete("/<tid>")]
pub fn delete(user: User, state: State<AppState>, db: Database, tid: i64) -> super::Result<Json<()>> {
    let delete_tx = DeleteTransaction::new(tid, user.user_id);
    match delete_tx.execute(&*db) {
        Ok(true) => Ok(Json(())),
        Ok(false) => Err(not_found("transaction")),
        Err(err) => {
            debug!(&state.log, "Error deleting transaction from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}
