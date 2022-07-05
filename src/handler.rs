use crate::types::{Cli, Writer};
use aws_sdk_s3::Client;
use log::info;


pub struct Handler {
    writer: Box<dyn Writer>,
    client: Client,
}

impl Handler {
    pub fn new(writer: Box<dyn Writer>, client: Client) -> Handler {
        Handler {
            writer,
            client,
        }
    }

    pub async fn handle(&self, args: Cli) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = args.endpoint;
        let pattern = args.pattern;
        let kms_key = args.kms_key;
        let bucket = args.bucket;

        // FIXME: Unstable
        let resp = self.client.list_objects_v2().bucket(&bucket).send().await?;

        for object in resp.contents().unwrap_or_default() {
            info!("Object key ({}).", object.key().unwrap_or_default());
        }
        // END of unstable code.

        info!("Searching files into Bucket ({}) of Endpoint ({}) with Pattern ({}) and KMS \
        Key ({}).", bucket, endpoint, pattern, kms_key);

        Ok(())
    }
}