use gix_hash::ObjectId;
use snafu::{NoneError, OptionExt, ResultExt};
use tokio::fs;
use tokio::process::Command;
use tracing::info;

use crate::consumer::PrintConsumer;
use crate::error::{
    boxed, CloneRepoSnafu, ConvertObjectIdSnafu, FeedResult, FileSystemSnafu, GeneralSnafu,
    InvalidNumberSnafu, InvalidUtf8Snafu, PullRepoSnafu, RunCommandSnafu,
};
use crate::local::{FetchRequest, FetchTask};

#[derive(Debug, Clone)]
pub struct ServerState {
    repo_dir: String,
}

impl ServerState {
    pub async fn new(repo_dir: String) -> FeedResult<Self> {
        fs::create_dir_all(&repo_dir)
            .await
            .context(FileSystemSnafu)?;

        Ok(Self { repo_dir })
    }

    pub async fn is_repo_exist(&self, org: &str, repo: &str) -> FeedResult<bool> {
        let path = self.repo_path(org, repo);
        fs::try_exists(&path).await.context(FileSystemSnafu)
    }

    /// Compose the repo name
    fn repo_name(&self, org: &str, repo: &str) -> String {
        format!("{}-{}", org, repo)
    }

    /// Get the repo path without checking if it exists
    pub fn repo_path(&self, org: &str, repo: &str) -> String {
        format!("{}/{}", self.repo_dir, self.repo_name(org, repo))
    }

    // todo: use gix to clone/update repo
    pub async fn clone_repo(&self, org: &str, repo: &str) -> FeedResult<()> {
        let output = Command::new("git")
            .arg("clone")
            .arg(format!("https://github.com/{}/{}.git", org, repo))
            .arg(self.repo_name(org, repo))
            .current_dir(&self.repo_dir)
            .output()
            .await
            .with_context(|_| RunCommandSnafu {
                command: "git clone",
            })?;

        // todo: git clone seems to return before it finish
        info!(
            "git clone {org}/{repo} status: {}, stderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );

        self.is_repo_exist(org, repo)
            .await?
            .then_some(())
            .with_context(|| CloneRepoSnafu { org, repo })
    }

    // todo: use gix to clone/update repo
    pub async fn pull_repo(&self, org: &str, repo: &str) -> FeedResult<()> {
        let output = Command::new("git")
            .arg("pull")
            .current_dir(format!("{}/{}", self.repo_dir, self.repo_name(org, repo)))
            .output()
            .await
            .with_context(|_| RunCommandSnafu {
                command: "git pull",
            })?;

        info!("git pull {org}/{repo} status: {}", output.status);

        output
            .status
            .success()
            .then_some(())
            .with_context(|| PullRepoSnafu { org, repo })
    }

    pub async fn head_commit(&self, org: &str, repo: &str) -> FeedResult<Vec<u8>> {
        let mut output = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .current_dir(format!("{}/{}", self.repo_dir, self.repo_name(org, repo)))
            .output()
            .await
            .with_context(|_| RunCommandSnafu {
                command: "git rev-parse HEAD",
            })?;

        // trim the tailing newline
        output.stdout.pop();
        Ok(output.stdout)
    }

    pub async fn count_commits(&self, org: &str, repo: &str) -> FeedResult<usize> {
        let mut output = Command::new("git")
            .arg("rev-list")
            .arg("--count")
            .arg("HEAD")
            .current_dir(format!("{}/{}", self.repo_dir, self.repo_name(org, repo)))
            .output()
            .await
            .with_context(|_| RunCommandSnafu {
                command: "git rev-list --count HEAD",
            })?;

        // trim the tailing newline
        output.stdout.pop();
        let output_string = String::from_utf8(output.stdout).context(InvalidUtf8Snafu)?;
        // trim the tailing newline
        output_string
            .parse()
            .map_err(|_| NoneError)
            .with_context(|_| InvalidNumberSnafu {
                number: output_string,
            })
    }

    pub async fn current_branch(&self, org: &str, repo: &str) -> FeedResult<String> {
        let mut output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .current_dir(format!("{}/{}", self.repo_dir, self.repo_name(org, repo)))
            .output()
            .await
            .with_context(|_| RunCommandSnafu {
                command: "git rev-parse --abbrev-ref HEAD",
            })?;

        // trim the tailing newline
        output.stdout.pop();
        String::from_utf8(output.stdout).context(InvalidUtf8Snafu)
    }

    pub async fn fetch_todo(
        &self,
        org: &str,
        repo: &str,
        since: Option<Vec<u8>>,
    ) -> FeedResult<()> {
        let since = if let Some(since) = since {
            Some(ObjectId::from_hex(&since).context(ConvertObjectIdSnafu)?)
        } else {
            None
        };

        let consumer = PrintConsumer {};

        let fetch_request = FetchRequest {
            root: self.repo_path(org, repo),
            branch: self.current_branch(org, repo).await?,
            since,
        };
        FetchTask::new(fetch_request)
            .map_err(boxed)
            .context(GeneralSnafu)?
            .execute(&consumer)
            .map_err(boxed)
            .context(GeneralSnafu)?;

        Ok(())
    }
}
