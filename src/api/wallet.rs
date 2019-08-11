/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Description |
/// | :--: | -- | -- | -- |
/// | POST | `/wallet` | `WalletCreationRequest` | create new wallet |
/// | GET | `/wallet/{wid}` | `id` | get wallet details |
/// | PUT | `/wallet/{wid}` | `WalletUpdateRequest` | update wallet details |
/// | DELETE | `/wallet/{wid}` | -- | delete wallet |
///
use rocket::{response::status, State};
use rocket_contrib::json::Json;
use slog::debug;

use kapitalist_types::request::{WalletCreationRequest, WalletUpdateRequest};
use kapitalist_types::response::WalletResponse;

use crate::api::util::{internal_server_error, not_found, update_request_invalid};
use crate::auth::User;
use crate::db::{
    wallet::{DeleteWallet, GetWallet, GetWalletsFromUser, NewWallet, UpdateWallet, Wallet},
    Database,
};
use crate::state::AppState;

#[post("/", data = "<req>")]
pub fn post(
    user: User,
    state: State<AppState>,
    db: Database,
    req: Json<WalletCreationRequest>,
) -> super::Result<status::Created<Json<WalletResponse>>> {
    let new_wallet = NewWallet::from_request(user.user_id, req.0);
    match new_wallet.execute(&*db) {
        Ok(wallet) => {
            let url = format!("/wallet/{}", wallet.id);
            Ok(status::Created(url, Some(Json(wallet.into_response()))))
        }
        Err(err) => {
            debug!(&state.log, "Error inserting wallet into database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[get("/<wid>")]
pub fn get(user: User, state: State<AppState>, db: Database, wid: i64) -> super::Result<Json<WalletResponse>> {
    let get_wallet = GetWallet::new(user.user_id, wid);
    match get_wallet.execute(&*db) {
        Ok(Ok(wallet)) => Ok(Json(wallet.into_response())),
        Ok(_) => Err(not_found("wallet")),
        Err(err) => {
            debug!(&state.log, "Error getting wallet from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[get("/all")]
pub fn get_all(user: User, state: State<AppState>, db: Database) -> super::Result<Json<Vec<WalletResponse>>> {
    let get_wallets = GetWalletsFromUser::new(user.user_id);
    match get_wallets.execute(&*db) {
        Ok(Some(wallets)) => Ok(Json(wallets.into_iter().map(Wallet::into_response).collect())),
        Ok(None) => Ok(Json(Vec::new())), // User has no wallets yet
        Err(err) => {
            debug!(&state.log, "Error getting wallets from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[put("/<wid>", data = "<req>")]
pub fn put(
    user: User,
    state: State<AppState>,
    db: Database,
    wid: i64,
    req: Json<WalletUpdateRequest>,
) -> super::Result<Json<WalletResponse>> {
    if !req.is_valid() {
        // At least one field has to be set, could also return 301 unchanged?
        return Err(update_request_invalid());
    }

    let update_wallet = UpdateWallet::from_request(user.user_id, wid, req.0);
    match update_wallet.execute(&*db) {
        Ok(Some(wallet)) => Ok(Json(wallet.into_response())),
        Ok(None) => Err(not_found("wallet")),
        Err(err) => {
            debug!(&state.log, "Error updating wallet in database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}

#[delete("/<wid>")]
pub fn delete(user: User, state: State<AppState>, db: Database, wid: i64) -> super::Result<Json<()>> {
    let delete_wallet = DeleteWallet::new(user.user_id, wid);
    match delete_wallet.execute(&*db) {
        Ok(true) => Ok(Json(())),
        Ok(false) => Err(not_found("wallet")),
        Err(err) => {
            debug!(&state.log, "Error deleting wallet from database"; "error" => %&err);
            Err(internal_server_error())
        }
    }
}
