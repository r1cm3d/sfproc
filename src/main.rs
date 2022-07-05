use log::{error, LevelFilter};
use simple_logger::SimpleLogger;
use std::process::exit;
use infra::writer::Fs;
use handler::Handler;
use types::Cli;
use clap::Parser;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use std::error::Error;

mod handler;
mod infra;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    // FIXME: Unstable
    let region_provider = RegionProviderChain::default_provider().or_else("sa-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let args = Cli::parse();
    let fs_writer = Box::new(Fs {});
    let handler = Handler::new(fs_writer, client);

    match handler.handle(args).await {
        Ok(..) => exit(exitcode::OK),
        Err(e) => {
            error!("Something went wrong. Error: {}", e);
            exit(exitcode::USAGE)
        }
    }
}
