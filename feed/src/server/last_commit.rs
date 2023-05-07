use axum::extract::{Query, State};
use axum::{Form, Json};
use gix_hash::ObjectId;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt};

use crate::error::{ConvertObjectIdSnafu, FeedResult, MissingParameterSnafu};
use crate::server::state::ServerState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastCommitQuery {
    org: Option<String>,
    repo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LastCommitResponse {
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
    state: ServerState,
    query: LastCommitQuery,
    form: LastCommitQuery,
) -> FeedResult<LastCommitResponse> {
    let org = query
        .org
        .or(form.org)
        .with_context(|| MissingParameterSnafu { param: "org" })?;
    let repo = query
        .repo
        .or(form.repo)
        .with_context(|| MissingParameterSnafu { param: "repo" })?;
    let head_commit_bytes = state.head_commit(&org, &repo).await?;
    let head_object_id = ObjectId::from_hex(&head_commit_bytes).context(ConvertObjectIdSnafu)?;
    let head_commit = head_object_id.to_string();

    Ok(LastCommitResponse {
        last_commit: head_commit,
        ..Default::default()
    })
}
