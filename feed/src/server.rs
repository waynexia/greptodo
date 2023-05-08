mod last_commit;
mod some_files;
mod state;
mod update_repo;

use axum::http::Method;
use axum::{routing, Router};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use self::some_files::some_files;
use crate::server::last_commit::last_commit;
use crate::server::state::ServerState;
use crate::server::update_repo::update_repo;

pub async fn build_server(local_root: String) -> Router {
    let state = ServerState::new(local_root).await.unwrap();

    let router = Router::new()
        .route("/update_repo", routing::post(update_repo))
        .route("/last_commit", routing::post(last_commit))
        .route("/some_files", routing::post(some_files))
        .with_state(state);

    Router::new().nest("/api", router).layer(
        ServiceBuilder::new().layer(
            CorsLayer::new()
                .allow_methods([Method::POST])
                .allow_origin(Any),
        ),
    )
}
