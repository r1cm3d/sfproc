use clap::Parser;

pub trait Writer {
    fn write(&self, file_name: &str, content: &str) -> Option<Box<dyn std::error::Error>>;
}

#[derive(Debug, Parser)]
#[clap(name = "sfproc - settlement files processor")]
#[clap(version = "0.0.1")]
#[clap(about = "A CLI application that is responsible to process settlement files.", long_about = None)]
pub struct Cli {
    #[clap(short, long, required = true)]
    /// Endpoint/CIB related to the file.
    pub endpoint: String,

    #[clap(short, long, required = true)]
    /// S3 bucket to look up.
    pub bucket: String,

    #[clap(short, long, required = true)]
    /// The base file pattern to look up into storage repository.
    pub pattern: String,

    #[clap(short, long)]
    /// The ARN of the KMS key that must be used to encrypt the sensible files.
    pub kms_key: String,
}
