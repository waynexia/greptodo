use std::sync::OnceLock;

use gix::date::time::Format;
use gix::object::tree::diff::{Action, Change};
use gix::ThreadSafeRepository;
use gix_hash::ObjectId;
use regex::bytes::Regex;
use snafu::ResultExt;
use tracing::info;

use crate::consumer::Consumer;
use crate::error::{FeedResult, OpenRepoSnafu};
use crate::schema::{Operation, RecordBuilder};

pub const DEFAULT_REGEXS: &[&str] = &["(?i)//\\s*todo"];
static RE: OnceLock<Regex> = OnceLock::new();

#[derive(Debug)]
pub struct FetchRequest {
    /// Root path to the project. Parent path of `.git`
    pub root: String,
    /// The default path of one project
    pub branch: String,
    pub since: Option<ObjectId>,
    pub repo: String,
}

#[derive(Debug)]
pub struct FetchTask {
    repo: ThreadSafeRepository,
    since: Option<ObjectId>,
    req: FetchRequest,
}

impl FetchTask {
    pub fn new(req: FetchRequest) -> FeedResult<Self> {
        let repo = ThreadSafeRepository::discover(req.root.clone())
            .map_err(Box::new)
            .with_context(|_| OpenRepoSnafu {
                path: req.root.clone(),
            })?;

        // initialize the regex
        RE.get_or_init(|| Regex::new(DEFAULT_REGEXS[0]).unwrap());

        Ok(Self {
            repo,
            since: req.since,
            req,
        })
    }

    pub fn execute(&self, consumer: &dyn Consumer) -> FeedResult<()> {
        info!("executing request: {:?}", self.req);

        let tls_repo = self.repo.to_thread_local();
        let head_ref = tls_repo.head_ref().unwrap().unwrap();

        let mut curr_id = head_ref.id();
        loop {
            let mut ancestor = curr_id.ancestors().first_parent_only().all().unwrap();
            let parent = if let Some(Ok(parent)) = ancestor.nth(1) {
                parent
            } else {
                break;
            };

            // get parent tree to compute diff
            let parent_tree = parent.object().unwrap().into_commit().tree().unwrap();
            let commit = curr_id.object().unwrap().into_commit();
            let commit_id = curr_id.detach().to_string();

            // read commit info
            let author = commit.author().unwrap();
            let base_record = RecordBuilder::new_base(
                self.req.repo.clone(),
                commit.time().unwrap().format(Format::Unix),
                author.name.to_string(),
                author.email.to_string(),
                commit_id,
                commit.message().unwrap().title.to_string(),
            );

            // get and process diff
            let tree = commit.tree().unwrap();
            let _changes = parent_tree
                .changes()
                .unwrap()
                .for_each_to_obtain_tree(&tree, |changes| {
                    self.process_diff(&base_record, consumer, changes)
                });

            // stop on the given commit.
            if Some(curr_id.detach()) == self.since {
                break;
            }
            curr_id = parent;
        }

        Ok(())
    }

    fn process_diff(
        &self,
        base_record: &RecordBuilder,
        consumer: &dyn Consumer,
        changes: Change,
    ) -> FeedResult<Action> {
        let location = changes.location.to_string();
        let location = if location.is_empty() {
            None
        } else {
            Some(location)
        };

        let diff = if let Some(Ok(diff)) = changes.event.diff() {
            diff
        } else {
            return Ok(Action::Continue);
        };

        diff.lines(|changes| -> Result<(), !> {
            let re = RE.get().unwrap();
            match changes {
                gix::object::blob::diff::line::Change::Addition { lines } => {
                    for line in lines {
                        if re.is_match(line) {
                            let record = base_record.build(
                                Operation::Add,
                                location.clone(),
                                line.to_string(),
                            );
                            consumer.record(record);
                        }
                    }
                }
                gix::object::blob::diff::line::Change::Deletion { lines } => {
                    for line in lines {
                        if re.is_match(line) {
                            let record = base_record.build(
                                Operation::Remove,
                                location.clone(),
                                line.to_string(),
                            );
                            consumer.record(record);
                        }
                    }
                }
                gix::object::blob::diff::line::Change::Modification {
                    lines_before,
                    lines_after,
                } => {
                    for line in lines_before {
                        if re.is_match(line) {
                            let record = base_record.build(
                                Operation::Remove,
                                location.clone(),
                                line.to_string(),
                            );
                            consumer.record(record);
                        }
                    }
                    for line in lines_after {
                        if re.is_match(line) {
                            let record = base_record.build(
                                Operation::Add,
                                location.clone(),
                                line.to_string(),
                            );
                            consumer.record(record);
                        }
                    }
                }
            }

            Ok(())
        })
        .unwrap();

        Ok(Action::Continue)
    }
}
