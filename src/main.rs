extern crate core;

use log::{error, LevelFilter};
use simple_logger::SimpleLogger;
use std::process::exit;
use handler::Handler;
use types::Cli;
use clap::Parser;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

mod handler;
mod types;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();

    let log_level = match args.verbose {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    SimpleLogger::new()
        .with_level(log_level)
        .init()
        .unwrap();

    let region_provider = RegionProviderChain::default_provider().or_else("sa-east-1");
    let config = aws_config::from_env()
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);
    let handler = Handler::new(client);

    match handler.handle(args).await {
        Ok(..) => exit(exitcode::OK),
        Err(e) => {
            error!("Something went wrong. Error: {}", e);
            exit(exitcode::USAGE)
        }
    }
}
