pub use snafu::prelude::*;
use snafu::Location;
pub use snafu::{Backtrace, ErrorCompat};

pub type FeedResult<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("At {location}."))]
    General {
        source: Box<dyn std::error::Error + Send + Sync>,
        location: Location,
    },

    #[snafu(display("At {location}. File system IO error: {source}"))]
    FileSystem {
        source: std::io::Error,
        location: Location,
    },

    #[snafu(display("At {location}. Failed to convert object id: {source}"))]
    ConvertObjectId {
        source: gix_hash::decode::Error,
        location: Location,
    },

    #[snafu(display("At {location}. Cannot find commit {commit}"))]
    CommitNotFound { commit: String, location: Location },

    #[snafu(display("At {location}. Failed to open repo at {path}"))]
    OpenRepo { path: String, location: Location },
}

pub fn boxed<E: std::error::Error + Send + Sync + 'static>(
    source: E,
) -> Box<dyn std::error::Error + Send + Sync> {
    Box::new(source)
}
