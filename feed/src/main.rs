#![feature(byte_slice_trim_ascii)]
#![feature(never_type)]
#![feature(trivial_bounds)]
#![feature(once_cell)]

use std::net::SocketAddr;

use clap::Parser;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::FeedConfig;

mod config;
mod conn;
mod consumer;
mod error;
mod local;
mod schema;
mod server;

#[tokio::main]
async fn main() {
    let config = FeedConfig::parse();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new(config.log_level.to_string()))
        .init();

    info!("{config:?}");
    let addr = SocketAddr::new(config.addr.parse().unwrap(), config.port);
    let app = server::build_server(config.repo_dir).await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
