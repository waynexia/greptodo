use std::str::FromStr;
use std::sync::OnceLock;

use gix::date::time::Format;
use gix::object::tree::diff::{Action, Change};
use gix::ThreadSafeRepository;
use gix_diff::tree::Changes;
use gix_hash::ObjectId;
use gix_odb::{Find, FindExt, Handle};
use gix_traverse::commit::{ancestors, Ancestors};
use regex::bytes::Regex;
use snafu::{OptionExt, ResultExt};

use crate::consumer::Consumer;
use crate::error::{
    self, CommitNotFoundSnafu, ConvertObjectIdSnafu, FeedResult, FileSystemSnafu, GeneralSnafu,
    OpenRepoSnafu,
};
use crate::schema::{Operation, Record, RecordBuilder};

pub const DEFAULT_REGEXS: &[&str] = &["(?i)//\\s*todo"];
static RE: OnceLock<Regex> = OnceLock::new();

pub struct FetchRequest {
    /// Root path to the project. Parent path of `.git`
    root: String,
    /// The default path of one project
    branch: String,
    since: Option<ObjectId>,
}

pub struct FetchTask {
    repo: ThreadSafeRepository,
    since: Option<ObjectId>,
    req: FetchRequest,
}

impl FetchTask {
    pub fn new(req: FetchRequest) -> FeedResult<Self> {
        let repo =
            ThreadSafeRepository::discover(req.root.clone()).with_context(|_| OpenRepoSnafu {
                path: req.root.clone(),
            })?;

        // initialize the regex
        RE.get_or_init(|| Regex::new(DEFAULT_REGEXS[0]).unwrap());

        Ok(Self {
            repo,
            since: req.since.clone(),
            req,
        })
    }

    pub fn execute(&self, consumer: &dyn Consumer) -> FeedResult<()> {
        let tls_repo = self.repo.to_thread_local();
        let head_ref = tls_repo.head_ref().unwrap().unwrap();

        let mut curr_id = head_ref.id();
        loop {
            let ancestor = curr_id.ancestors().first_parent_only().all().unwrap();
            let parent = if let Some(Ok(parent)) = ancestor.skip(1).next() {
                parent
            } else {
                break;
            };

            // get parent tree to compute diff
            let parent_tree = parent.object().unwrap().into_commit().tree().unwrap();
            let commit = curr_id.object().unwrap().into_commit();
            let commit_id = curr_id.clone().detach().to_string();

            // read commit info
            let author = commit.author().unwrap();
            let base_record = RecordBuilder::new_base(
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
            if Some(curr_id.clone().detach()) == self.since {
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

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use snafu::ErrorCompat;

    use super::*;
    use crate::consumer::PrintConsumer;
}
