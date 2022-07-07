use crate::types::{FILE_SUFFIX, EXTENSION_PATTERN, STREAMABLE_PATTERN, TENANT_PATTERN, DIR_PATTERN,
                   Cli, SettlementFile};
use crate::types::Metadata::{BackupFile, SourceFile, Endpoint, Tenant, Streamable, ParentCid};
use aws_sdk_s3::Client;
use aws_sdk_s3::model::{MetadataDirective, ServerSideEncryption};
use log::{debug, error, info};
use futures::stream::TryStreamExt;
use regex::Regex;
use uuid::Uuid;

pub struct Handler {
    client: Client,
    file_suffix: String,
    page_size: i32,
    extension_pattern: Regex,
    tenant_pattern: Regex,
    streamable_pattern: Regex,
    dir_pattern: Regex,
}

impl Handler {
    pub fn new(client: Client) -> Handler {
        Handler {
            client,
            file_suffix: FILE_SUFFIX.to_string(),
            page_size: 10,
            extension_pattern: Regex::new(EXTENSION_PATTERN).unwrap(),
            tenant_pattern: Regex::new(TENANT_PATTERN).unwrap(),
            streamable_pattern: Regex::new(STREAMABLE_PATTERN).unwrap(),
            dir_pattern: Regex::new(DIR_PATTERN).unwrap(),
        }
    }

    pub async fn handle(&self, args: Cli) -> Result<(), String> {
        let endpoint = args.endpoint;
        let regex = args.regex;
        let kms_key = args.kms_key;
        let bucket = args.bucket;
        let prefix = args.prefix;
        let pretend = args.pretend;

        let mut stream = self.client.list_objects_v2()
            .bucket(&bucket)
            .prefix(prefix)
            .into_paginator()
            .page_size(self.page_size)
            .send();

        let mut ack_files = Vec::new();

        while let Some(resp) = stream
            .try_next()
            .await
            .map_err(|err| err.to_string())? {

            for object in resp.contents().unwrap_or_default() {
                let file_name = object.key().unwrap_or_default();
                info!("Key ({}) has been acknowledged.", file_name);
                ack_files.push(file_name.to_string());
            }
        }

        let filtered_files = ack_files.into_iter()
            .filter(|obj_key| {
                self.filter(obj_key, &regex)
            })
            .map(|obj_key| {
                self.to_settlement_file(&endpoint, &bucket, &obj_key)
            })
            .filter(|sf| {
                if pretend {
                   info!("Pretend mode is enable. Settlement File ({}) will not be copied.", sf)
                }

                !pretend
            })
            .collect::<Vec<SettlementFile>>();

        for sf in filtered_files.into_iter() {
            match self.copy_object(&sf, &kms_key).await {
                Ok(_) => info!("Settlement File ({}) has successfully processed.", sf),
                Err(err) => error!("Settlement File ({}) has not processed. Error: {}", sf, err)
            }
        }

        Ok(())
    }

    fn filter(&self, obj_key: &str, regex: &Option<String>) -> bool {
        if !self.extension_pattern.is_match(&obj_key) ||
            !self.tenant_pattern.is_match(&obj_key) ||
            self.dir_pattern.is_match(&obj_key) {
            debug!("Skipping key({}).", obj_key);
            return false;
        }

        if regex.is_none() {
            info!("File ({}) will be copied without any regular expression pattern.", obj_key);
            return true;
        }

        let regex = &regex.as_ref().unwrap();
        let input_pattern = Regex::new(regex.as_ref());

        if input_pattern.is_err() {
            error!("Cannot compile regular expression ({}). Skipping key ({}).", regex, obj_key);
            return false;
        }

        let input_pattern = input_pattern.unwrap();

        info!("Filtering only files that matches with the Pattern ({}).", regex);
        let is_match = input_pattern.is_match(obj_key);

        if is_match {
            info!("File ({}) will be copied since it matches ({}) regular expression pattern", obj_key, regex);
        }

        return is_match;
    }

    fn to_settlement_file(&self, endpoint: &str, bucket: &str, obj_key: &str) -> SettlementFile {
        debug!("File to apply regex ({}).", obj_key);
        let ext = self.extension_pattern.find(&obj_key).unwrap().as_str();
        debug!("ext: ({}).", ext);
        let tenant = self.tenant_pattern.find(&obj_key).unwrap().as_str();
        debug!("tenant: ({}).", tenant);
        let streamable = self.streamable_pattern.is_match(&obj_key);
        debug!("streamable: ({}).", streamable);
        let new_name = obj_key.replace(ext, "");
        let new_name = format!("{}{}{}", new_name, self.file_suffix, ext);

        SettlementFile {
            bucket: bucket.to_string(),
            tenant: Tenant(tenant.to_string()),
            endpoint: Endpoint(endpoint.to_string()),
            source_file: SourceFile(obj_key.to_string()),
            backup_file: BackupFile(new_name),
            streamable: Streamable(streamable.to_string()),
            parent_cid: ParentCid(Uuid::new_v4().to_string()),
        }
    }

    async fn copy_object(&self, sf: &SettlementFile, kms_key: &Option<String>) ->
    Result<(), String> {
        let source_file = format!("{}/{}", sf.bucket, sf.source_file.value());
        debug!("Source File ({})", source_file);
        info!("Copying Settlement File ({}).", sf);

        let req = self.client.copy_object()
            .copy_source(source_file)
            .bucket(&sf.bucket)
            .key(&sf.backup_file.value())
            .metadata_directive(MetadataDirective::Replace)
            .metadata(sf.tenant.name(), sf.tenant.value())
            .metadata(sf.endpoint.name(), sf.endpoint.value())
            .metadata(sf.source_file.name(), sf.source_file.value())
            .metadata(sf.backup_file.name(), sf.backup_file.value())
            .metadata(sf.streamable.name(), sf.streamable.value())
            .metadata(sf.parent_cid.name(), sf.parent_cid.value());

        let req = if sf.streamable.value().eq(true.to_string().as_str()) {
            if kms_key.is_none() {
                return Err(format!("KMS not found for Streamable file ({})", sf));
            }

            let kms_key = kms_key.as_ref().unwrap();

            info!("Since file ({}) is streamable. It will be encrypted with KMS key ({}).", sf,
                kms_key);
            req.server_side_encryption(ServerSideEncryption::AwsKms)
                .ssekms_key_id(kms_key)
        } else {
            req
        };

        let output = req
            .send()
            .await
            .map_err(|err| err.to_string())?;

        if output.copy_object_result.is_none() {
            return Err("Something went wrong copying the object. Use verbose mode (--verbose) for more details".to_string());
        }

        Ok(())
    }
}