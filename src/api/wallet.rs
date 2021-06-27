/// from doc/api.md
///
/// | Method | Endpoint | Payload/Params | Description |
/// | :--: | -- | -- | -- |
/// | POST | `/wallet` | `WalletCreationRequest` | create new wallet |
/// | GET | `/wallet/{wid}` | `id` | get wallet details |
/// | PUT | `/wallet/{wid}` | `WalletUpdateRequest` | update wallet details |
/// | DELETE | `/wallet/{wid}` | -- | delete wallet |
/// | GET | `/wallet/all` | `from, to` | get transaction history |
///
use tracing::debug;
use warp::{reject, reply, Rejection, Reply};

use kapitalist_types::request::{WalletCreationRequest, WalletUpdateRequest};
use kapitalist_types::response::WalletResponse;

use crate::api::util;
use crate::auth::User;
use crate::db::{
    wallet::{DeleteWallet, GetWallet, GetWalletsFromUser, NewWallet, UpdateWallet, Wallet},
    Database,
};

pub async fn post(
    db: Database,
    user: User,
    req: WalletCreationRequest,
) -> Result<impl Reply, Rejection> {
    let new_wallet = NewWallet::from_request(user.user_id, req);
    match new_wallet.execute(&*db.0) {
        Ok(wallet) => {
            let url = format!("/wallet/{}", wallet.id);
            Ok(util::created(&wallet.into_response(), url))
        }
        Err(err) => {
            debug!(error = %&err, "Error inserting wallet into database");
            Err(reject::reject())
        }
    }
}

pub async fn get(db: Database, user: User, wid: i64) -> Result<impl Reply, Rejection> {
    let get_wallet = GetWallet::new(user.user_id, wid);
    match get_wallet.execute(&*db.0) {
        Ok(Ok(wallet)) => Ok(reply::json(&wallet.into_response())),
        Ok(_) => Err(util::not_found("wallet")),
        Err(err) => {
            debug!(error = %&err, "Error getting wallet from database");
            Err(reject::reject())
        }
    }
}

pub async fn put(
    db: Database,
    user: User,
    wid: i64,
    req: WalletUpdateRequest,
) -> Result<impl Reply, Rejection> {
    if !req.is_valid() {
        // At least one field has to be set, could also return 301 unchanged?
        return Err(reject::reject());
    }

    let update_wallet = UpdateWallet::from_request(user.user_id, wid, req);
    match update_wallet.execute(&*db.0) {
        Ok(Some(wallet)) => Ok(reply::json(&wallet.into_response())),
        Ok(None) => Err(util::not_found("wallet")),
        Err(err) => {
            debug!(error = %&err, "Error updating wallet in database");
            Err(reject::reject())
        }
    }
}

pub async fn delete(db: Database, user: User, wid: i64) -> Result<impl Reply, Rejection> {
    let delete_wallet = DeleteWallet::new(user.user_id, wid);
    match delete_wallet.execute(&*db.0) {
        Ok(true) => Ok(reply::json(&())),
        Ok(false) => Err(util::not_found("wallet")),
        Err(err) => {
            debug!(error = %&err, "Error deleting wallet from database");
            Err(util::error(err))
        }
    }
}

pub async fn all(db: Database, user: User) -> Result<impl Reply, Rejection> {
    let get_wallets = GetWalletsFromUser::new(user.user_id);
    let resp: Vec<WalletResponse> = match get_wallets.execute(&*db.0) {
        // collect user's wallets
        Ok(Some(wallets)) => wallets.into_iter().map(Wallet::into_response).collect(),
        // user has no wallets yet
        Ok(None) => vec![],
        // db error
        Err(err) => {
            debug!(error = %&err, "Error getting wallets from database");
            return Err(util::error(err));
        }
    };
    Ok(reply::json(&resp))
}
