use std::string::FromUtf8Error;

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
    OpenRepo {
        path: String,
        location: Location,
        source: gix::discover::Error,
    },

    #[snafu(display("At {location}. Failed to run command `{command}`: {source}"))]
    RunCommand {
        command: String,
        location: Location,
        source: std::io::Error,
    },

    #[snafu(display("At {location}. Failed to clone repo {org}/{repo}"))]
    CloneRepo {
        org: String,
        repo: String,
        location: Location,
    },

    #[snafu(display("At {location}. Failed to pull repo {org}/{repo}"))]
    PullRepo {
        org: String,
        repo: String,
        location: Location,
    },

    #[snafu(display("At {location}. Failed to get head commit of {org}/{repo}"))]
    HeadCommit {
        org: String,
        repo: String,
        location: Location,
    },

    #[snafu(display("At {location}. Missing parameter {param}"))]
    MissingParameter { param: String, location: Location },

    #[snafu(display("At {location}. Invalid UTF-8 string"))]
    InvalidUtf8 {
        location: Location,
        source: FromUtf8Error,
    },

    #[snafu(display("At {location}. Invalid number: {number}"))]
    InvalidNumber { number: String, location: Location },
}

pub fn boxed<E: std::error::Error + Send + Sync + 'static>(
    source: E,
) -> Box<dyn std::error::Error + Send + Sync> {
    Box::new(source)
}
