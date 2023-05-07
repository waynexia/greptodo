//! Retrieve some random files from the given repository.

use axum::extract::{Query, State};
use axum::{Form, Json};
use gix::features::fs::WalkDir;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt};

use crate::error::{FeedResult, FileSystemSnafu, MissingParameterSnafu};
use crate::server::state::ServerState;

const MIN_FILE_SIZE: u64 = 512;
const MAX_FILE_SIZE: u64 = 4 * 1024;
const DEFAULT_FILE_NUM: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomeFilesQuery {
    org: Option<String>,
    repo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SomeFilesResponse {
    num_files: usize,
    files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn some_files(
    State(state): State<ServerState>,
    Query(query): Query<SomeFilesQuery>,
    Form(form): Form<SomeFilesQuery>,
) -> Json<SomeFilesResponse> {
    let result = some_files_impl(state, query, form).await;
    let response = match result {
        Ok(resp) => resp,
        Err(e) => SomeFilesResponse {
            error: Some(e.to_string()),
            ..Default::default()
        },
    };

    Json(response)
}

async fn some_files_impl(
    state: ServerState,
    query: SomeFilesQuery,
    form: SomeFilesQuery,
) -> FeedResult<SomeFilesResponse> {
    let org = query
        .org
        .or(form.org)
        .with_context(|| MissingParameterSnafu { param: "org" })?;
    let repo = query
        .repo
        .or(form.repo)
        .with_context(|| MissingParameterSnafu { param: "repo" })?;

    let repo_path = state.repo_path(&org, &repo);
    let mut entries = WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.metadata().unwrap().len() >= MIN_FILE_SIZE
                && e.metadata().unwrap().len() <= MAX_FILE_SIZE
        })
        .map(|e| e.path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    entries.shuffle(&mut rng);
    let file_names = entries.into_iter().take(DEFAULT_FILE_NUM);

    let mut files = Vec::with_capacity(DEFAULT_FILE_NUM);
    for file_name in file_names {
        let content = std::fs::read_to_string(&file_name).context(FileSystemSnafu)?;
        files.push(content);
    }

    Ok(SomeFilesResponse {
        num_files: files.len(),
        files,
        ..Default::default()
    })
}
