use log::{error, LevelFilter};
use simple_logger::SimpleLogger;
use std::process::exit;
use infra::writer::Fs;
use handler::Handler;
use types::Cli;
use clap::Parser;

mod handler;
mod infra;
mod types;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    let args = Cli::parse();
    let fs_writer = Box::new(Fs {});
    let handler = Handler::new(fs_writer);

    match handler.handle(args) {
        Ok(..) => exit(exitcode::OK),
        Err(e) => {
            error!("Something went wrong. Error: {}", e);
            exit(exitcode::USAGE)
        }
    }
}
