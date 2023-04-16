use std::str::FromStr;

use gix::date::time::Format;
use gix::object::tree::diff::{Action, Change};
use gix::ThreadSafeRepository;
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
use crate::schema::{Operation, Record, RecordBuilder};

pub struct FetchRequest {
    /// Root path to the project. Parent path of `.git`
    root: String,
    /// The default path of one project
    branch: String,
    since: Option<ObjectId>,
}

pub struct FetchTask {
    repo: ThreadSafeRepository,
    since: ObjectId,
    req: FetchRequest,
}

impl FetchTask {
    pub fn new(req: FetchRequest) -> FeedResult<Self> {
        let repo =
            ThreadSafeRepository::discover(req.root.clone()).with_context(|_| OpenRepoSnafu {
                path: req.root.clone(),
            })?;

        Ok(Self {
            repo,
            since: req.since.clone().unwrap(),
            req,
        })
    }

    pub fn execute(&self, consumer: &dyn Consumer) -> FeedResult<()> {
        let tls_repo = self.repo.to_thread_local();
        let head_ref = tls_repo.head_ref().unwrap().unwrap();

        let mut curr_id = head_ref.id();
        loop {
            println!("{curr_id}");

            let ancestor = curr_id.ancestors().first_parent_only().all().unwrap();
            let parent = if let Some(Ok(parent)) = ancestor.skip(1).next() {
                parent
            } else {
                break;
            };

            let parent_tree = parent.object().unwrap().into_commit().tree().unwrap();
            let commit = curr_id.object().unwrap().into_commit();

            let author = commit.author().unwrap();
            let base_record = RecordBuilder::new_base(
                commit.time().unwrap().format(Format::Unix),
                author.name.to_string(),
                author.email.to_string(),
                commit.message().unwrap().title.to_string(),
            );
            println!("commit message: {:?}", base_record);

            let tree = commit.tree().unwrap();
            let changes = parent_tree
                .changes()
                .unwrap()
                .for_each_to_obtain_tree(&tree, |changes| self.process_diff(&base_record, changes));

            if curr_id.clone().detach() == self.since {
                break;
            }
            curr_id = parent;

            // break;
        }

        Ok(())
    }

    fn process_diff(&self, base_record: &RecordBuilder, changes: Change) -> FeedResult<Action> {
        let location = changes.location.to_string();
        println!("\n=================================================");
        println!("location: {}", location);

        let diff = if let Some(Ok(diff)) = changes.event.diff() {
            diff
        } else {
            return Ok(Action::Continue);
        };
        diff.lines(|changes| -> Result<(), !> {
            match changes {
                gix::object::blob::diff::line::Change::Addition { lines } => {
                    println!("+++ {:?}", lines)
                }
                gix::object::blob::diff::line::Change::Deletion { lines } => {
                    println!("--- {:?}", lines)
                }
                gix::object::blob::diff::line::Change::Modification {
                    lines_before,
                    lines_after,
                } => println!("??? --- {:?}\n??? +++ {:?}", lines_before, lines_after),
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
