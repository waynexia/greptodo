use axum::extract::{Query, State};
use axum::{Form, Json};
use serde::{Deserialize, Serialize};
use snafu::OptionExt;

use crate::error::{FeedResult, MissingParameterSnafu};
use crate::server::state::ServerState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastCommitQuery {
    org: Option<String>,
    repo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastCommitResponse {
    repo_exist: bool,
    last_commit: String,
}

#[axum_macros::debug_handler]
pub async fn last_commit(
    State(state): State<ServerState>,
    Query(query): Query<LastCommitQuery>,
    Form(form): Form<LastCommitQuery>,
) -> Json<LastCommitResponse> {
    todo!()
}

fn last_commit_impl(
    State(state): State<ServerState>,
    Query(query): Query<LastCommitQuery>,
    Form(form): Form<LastCommitQuery>,
) -> FeedResult<LastCommitResponse> {
    let org = query
        .org
        .or(form.org)
        .with_context(|| MissingParameterSnafu { param: "org" })?;
    let repo = query
        .repo
        .or(form.repo)
        .with_context(|| MissingParameterSnafu { param: "repo" })?;

    todo!()
}
