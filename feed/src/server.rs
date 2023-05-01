mod last_commit;
mod state;
mod update_repo;

use axum::{routing, Router};

use crate::server::last_commit::last_commit;
use crate::server::state::ServerState;
use crate::server::update_repo::update_repo;

pub async fn build_server(local_root: String) -> Router {
    let state = ServerState::new(local_root).await.unwrap();

    let router = Router::new()
        .route("/update_repo", routing::post(update_repo))
        .route("/last_commit", routing::post(last_commit))
        .with_state(state);

    Router::new().nest("/api", router)
}
