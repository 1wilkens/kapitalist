use std::convert::Infallible;
use std::sync::Arc;

use warp::Filter;

use crate::cfg::Config;

pub fn attach(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

// XXX: Maybe implement debug for this
pub struct AppState {
    pub(crate) config: Config,
}

pub struct AppStateBuilder {
    pub(crate) config: Config,
}

impl AppState {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(cfg: Config) -> AppStateBuilder {
        AppStateBuilder { config: cfg }
    }
}

impl AppStateBuilder {
    pub fn build(self) -> AppState {
        AppState {
            config: self.config,
        }
    }
}
