use clap::Parser;
use std::fmt;

pub const FILE_SUFFIX: &str = "";
pub const EXTENSION_PATTERN: &str = r"\..{3}$";
pub const TENANT_PATTERN: &str = r"(?i)^tn-[^/]+";
pub const STREAMABLE_PATTERN: &str = r"(?i).*(baseii|t112|t120|t470|t464).*";
pub const DIR_PATTERN: &str = r".*/&";

#[derive(Debug, Parser)]
#[clap(name = "sfproc - settlement files processor")]
#[clap(version = "0.2.3")]
#[clap(about = "A CLI application that is responsible to process settlement files.", long_about = None)]
pub struct Cli {
    #[clap(short, long, required = true)]
    /// Endpoint/CIB related to the file.
    pub endpoint: String,

    #[clap(short, long, required = true)]
    /// S3 bucket to look up.
    pub bucket: String,

    #[clap(short, long)]
    /// The prefix to be applied in the look up in order to avoid unnecessary requests.
    pub prefix: String,

    #[clap(short, long)]
    /// The suffix to be applied in the end of the file name. This option is DANGEROUS and might skip the integrity validation.
    pub suffix: Option<String>,

    #[clap(short, long)]
    /// The base regex pattern to look up into storage repository.
    pub regex: Option<String>,

    #[clap(short, long)]
    /// The ARN of the KMS key that must be used to encrypt the sensible files.
    pub kms_key: Option<String>,

    #[clap(short, long, action)]
    /// Enable DEBUG log mode.
    pub verbose: bool,

    #[clap(long, action)]
    /// Enable pretend mode. In the pretend mode, the files will not be copied. This option is
    /// useful to validate the --regex option.
    pub pretend: bool,
}

pub enum Metadata {
    SourceFile(String),
    BackupFile(String),
    Tenant(String),
    Streamable(String),
    ParentCid(String),
    Endpoint(String),
}

impl Metadata {
    pub fn name(&self) -> String {
        match *self {
            Metadata::SourceFile(_) => String::from("SourceFile"),
            Metadata::BackupFile(_) => String::from("BackupFile"),
            Metadata::Tenant(_) => String::from("OrgId"),
            Metadata::Endpoint(_) => String::from("Endpoint"),
            Metadata::Streamable(_) => String::from("Streamable"),
            Metadata::ParentCid(_) => String::from("ParentCid"),
        }
    }

    pub fn value(&self) -> String {
        match self {
            Metadata::SourceFile(v) => v.to_string(),
            Metadata::BackupFile(v) => v.to_string(),
            Metadata::Tenant(v) => v.to_string(),
            Metadata::Endpoint(v) => v.to_string(),
            Metadata::Streamable(v) => v.to_string(),
            Metadata::ParentCid(v) => v.to_string(),
        }
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ({})", self.name(), self.value())
    }
}

pub struct SettlementFile {
    pub bucket: String,
    pub source_file: Metadata,
    pub backup_file: Metadata,
    pub tenant: Metadata,
    pub streamable: Metadata,
    pub endpoint: Metadata,
    pub parent_cid: Metadata,
}


impl fmt::Display for SettlementFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}; Bucket: ({}); {}; {}; {}; {}; {}", self.parent_cid, self.bucket, self.source_file,
               self.backup_file, self.tenant, self.streamable, self.endpoint)
    }
}