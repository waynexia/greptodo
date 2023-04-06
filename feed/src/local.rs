use std::str::FromStr;

use gix_diff::tree::Changes;
use gix_hash::ObjectId;
use gix_odb::{Find, FindExt, Handle};
use gix_traverse::commit::{ancestors, Ancestors};
use snafu::{OptionExt, ResultExt};

use crate::consumer::Consumer;
use crate::error::{
    self, CommitNotFoundSnafu, ConvertObjectIdSnafu, FeedResult, FileSystemSnafu, GeneralSnafu,
    OpenRepoSnafu,
};
use crate::schema::{Operation, Record};

pub struct FetchRequest {
    /// Root path to the project. Parent path of `.git`
    root: String,
    /// The default path of one project
    branch: String,
    since: Option<ObjectId>,
}

pub struct FetchTask {
    db: Handle,
    since: ObjectId,
    req: FetchRequest,
}

impl FetchTask {
    pub fn new(req: FetchRequest) -> Self {
        let db = gix_odb::at(&req.root)
            .ok()
            .context(OpenRepoSnafu {
                path: req.root.clone(),
            })
            .unwrap();

        Self {
            db,
            since: req.since.clone().unwrap(),
            req,
        }
    }

    pub fn execute(&self, consumer: &dyn Consumer) -> FeedResult<()> {
        let all_commits = self.find_commits(Some(self.since))?;

        for window in all_commits.as_slice().windows(2) {
            self.process_diff(window[0], window[1]).unwrap();
        }

        let mut buf = Vec::new();
        for oid in &all_commits {
            let commit = self.db.find_commit(oid, &mut buf).unwrap();
            let record = Record {
                commit_time: format!("{}", commit.time().seconds_since_unix_epoch),
                author_name: commit.author().name.to_string(),
                author_email: commit.author().email.to_string(),
                operation: Operation::Add,
                file_path: String::new(),
                commit_messaage: commit.message().summary().to_string(),
                content: String::new(),
                calc_time: String::new(),
            };
            consumer.record(record);
        }

        Ok(())
    }

    /// Find the HEAD object id of current repo
    fn find_head(&self) -> FeedResult<ObjectId> {
        let path = self
            .db
            .store_ref()
            .path()
            .parent()
            .unwrap()
            .join("refs")
            .join("heads")
            .join(&self.req.branch);
        let buf = std::fs::read(path).context(FileSystemSnafu)?;
        ObjectId::from_hex(&buf.trim_ascii_end()).context(ConvertObjectIdSnafu)
    }

    /// Find all commits before `until`. `until` is included
    fn find_commits(&self, until: Option<ObjectId>) -> FeedResult<Vec<ObjectId>> {
        let head = self
            .find_head()
            .map_err(error::boxed)
            .context(GeneralSnafu)?;

        let all_commits = Ancestors::new(Some(head), ancestors::State::default(), |oid, buf| {
            self.db.find_commit_iter(oid, buf)
        })
        .collect::<Result<Vec<_>, gix_traverse::commit::ancestors::Error>>()
        .map_err(error::boxed)
        .context(GeneralSnafu)?;

        // todo: improve this
        let filtered_commits = if let Some(until) = until {
            let mut result = Vec::new();
            for oid in all_commits {
                result.push(oid);
                if oid == until {
                    break;
                }
            }
            result
        } else {
            all_commits
        };

        Ok(filtered_commits)
    }

    fn process_diff(&self, before: ObjectId, after: ObjectId) -> FeedResult<()> {
        let mut before_buf = Vec::new();
        let mut after_buf = Vec::new();

        let before_commit = self
            .db
            .find_commit(&before, &mut before_buf)
            .unwrap()
            .tree();
        let after_commit = self.db.find_commit(&after, &mut after_buf).unwrap().tree();

        let before_tree = self
            .db
            .find_tree_iter(before_commit, &mut before_buf)
            .unwrap();
        let after_tree = self
            .db
            .find_tree_iter(after_commit, &mut after_buf)
            .unwrap();

        let mut recorder = gix_diff::tree::Recorder::default();
        Changes::from(before_tree)
            .needed_to_obtain(
                after_tree,
                &mut gix_diff::tree::State::default(),
                |oid, buf| {
                    // use gix_odb::pack::FindExt;
                    self.db.find_tree_iter(oid, buf)
                    // .map(|(obj, _)| obj.try_into_tree_iter().expect("only called for trees"))
                },
                &mut recorder,
            )
            .unwrap();

        println!("from {} to {}:", before, after);
        println!("{:#?}", recorder);

        // let before_tree = before_commit.tree().unwrap();
        // let after_tree = after_commit.tree().unwrap();

        // let diff = gix_diff::diff_tree(&self.db, &before_tree, &after_tree).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use snafu::ErrorCompat;

    use super::*;
    use crate::consumer::PrintConsumer;
}
