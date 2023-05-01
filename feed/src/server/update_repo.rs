use axum::extract::{Query, State};
use axum::{Form, Json};
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt};

use crate::error::{boxed, FeedResult, GeneralSnafu, MissingParameterSnafu};
use crate::server::state::ServerState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRepoQuery {
    org: Option<String>,
    repo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateRepoResponse {
    repo_exist: bool,
    num_new_commit: u64,
    num_todo_changes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn update_repo(
    State(state): State<ServerState>,
    Query(query): Query<UpdateRepoQuery>,
    Form(form): Form<UpdateRepoQuery>,
) -> Json<UpdateRepoResponse> {
    let result = update_repo_impl(state, query, form).await;
    let response = match result {
        Ok(resp) => resp,
        Err(e) => UpdateRepoResponse {
            error: Some(e.to_string()),
            ..Default::default()
        },
    };

    Json(response)
}

async fn update_repo_impl(
    state: ServerState,
    query: UpdateRepoQuery,
    form: UpdateRepoQuery,
) -> FeedResult<UpdateRepoResponse> {
    let org = query
        .org
        .or(form.org)
        .with_context(|| MissingParameterSnafu { param: "org" })?;
    let repo = query
        .repo
        .or(form.repo)
        .with_context(|| MissingParameterSnafu { param: "repo" })?;

    let mut curr_head = vec![];
    #[allow(unused_assignments)]
    let mut num_new_commit = 0;

    // clone or pull repo
    if state.is_repo_exist(&org, &repo).await? {
        curr_head = state.head_commit(&org, &repo).await?;
        let num_prev_commit = state.count_commits(&org, &repo).await?;
        state.pull_repo(&org, &repo).await?;
        let num_curr_commit = state.count_commits(&org, &repo).await?;
        num_new_commit = num_curr_commit - num_prev_commit;
    } else {
        state.clone_repo(&org, &repo).await?;
        num_new_commit = state.count_commits(&org, &repo).await?;
    }

    let repo_exist = !curr_head.is_empty();
    let curr_head = if curr_head.is_empty() {
        None
    } else {
        Some(curr_head)
    };

    state
        .fetch_todo(&org, &repo, curr_head)
        .await
        .map_err(boxed)
        .context(GeneralSnafu)?;

    Ok(UpdateRepoResponse {
        repo_exist,
        num_new_commit: num_new_commit as u64,
        // todo: count changes
        num_todo_changes: 0,
        error: None,
    })
}
