use crate::types::{Cli, Writer};
use log::info;

pub struct Handler {
    writer: Box<dyn Writer>,
}

impl Handler {
    pub fn new(writer: Box<dyn Writer>) -> Handler {
        Handler {
            writer,
        }
    }

    pub fn handle(&self, args: Cli) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = args.endpoint;
        let pattern = args.pattern;
        let kms_key = args.kms_key;
        let bucket = args.bucket;

        info!("Searching files into Bucket ({}) of Endpoint ({}) with Pattern ({}) and KMS \
        Key ({}).", bucket, endpoint, pattern, kms_key);

        Ok(())
    }
}