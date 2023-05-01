use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct FeedConfig {
    /// Directory to put cloned repositories
    #[arg(short, long, default_value = "/tmp/greptodo")]
    pub repo_dir: String,

    /// The log level, one of {error, warn, info, debug, trace}
    #[arg(short, long, default_value = "info")]
    pub log_level: tracing::Level,

    /// Address to bind
    #[arg(short, long, default_value = "0.0.0.0")]
    pub addr: String,

    /// Port to bind
    #[arg(short, long, default_value = "7531")]
    pub port: u16,
}
