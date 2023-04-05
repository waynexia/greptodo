use std::str::FromStr;

use gix_hash::ObjectId;
use gix_odb::{FindExt, Handle};
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

        let mut buf = Vec::new();
        for oid in &all_commits {
            let commit = self.db.find_commit(oid, &mut buf).unwrap();
            // println!("commit: {:?}", commit);
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
            // println!("{record:#?}")
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
}
