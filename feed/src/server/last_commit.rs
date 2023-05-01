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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LastCommitResponse {
    repo_exist: bool,
    last_commit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn last_commit(
    State(state): State<ServerState>,
    Query(query): Query<LastCommitQuery>,
    Form(form): Form<LastCommitQuery>,
) -> Json<LastCommitResponse> {
    let result = last_commit_impl(state, query, form).await;
    let response = match result {
        Ok(resp) => resp,
        Err(e) => LastCommitResponse {
            error: Some(e.to_string()),
            ..Default::default()
        },
    };

    Json(response)
}

async fn last_commit_impl(
    _state: ServerState,
    query: LastCommitQuery,
    form: LastCommitQuery,
) -> FeedResult<LastCommitResponse> {
    let _org = query
        .org
        .or(form.org)
        .with_context(|| MissingParameterSnafu { param: "org" })?;
    let _repo = query
        .repo
        .or(form.repo)
        .with_context(|| MissingParameterSnafu { param: "repo" })?;

    todo!()
}
