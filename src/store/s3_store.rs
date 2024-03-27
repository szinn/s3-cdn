use async_trait::async_trait;
use bytes::Bytes;
use minio::s3::{
    args::{BucketExistsArgs, ObjectConditionalReadArgs},
    client::Client,
    creds::StaticProvider,
    http::BaseUrl,
};

use crate::{core::store_service::StoreService, error::Error};

struct S3Config {
    server: String,
    #[allow(dead_code)]
    region: String,
    bucket: String,
    access_key_id: String,
    secret_access_key: String,
}

impl S3Config {
    fn new(server: &str, region: &str, bucket: &str, access_key_id: &str, secret_access_key: &str) -> Self {
        Self {
            server: server.to_string(),
            region: region.to_string(),
            bucket: bucket.to_string(),
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string(),
        }
    }
}
pub struct S3Store {
    config: S3Config,
}

impl S3Store {
    pub fn new(server: &str, region: &str, bucket: &str, access_key_id: &str, secret_access_key: &str) -> Self {
        let config = S3Config::new(server, region, bucket, access_key_id, secret_access_key);
        Self { config }
    }
}

#[async_trait]
impl StoreService for S3Store {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_object(&self, path: &str) -> Result<Bytes, Error> {
        let config = &self.config;
        let base_url: BaseUrl = config.server.parse::<BaseUrl>()?;

        tracing::info!("Trying to connect to MinIO at: `{:?}`", base_url);

        let static_provider = StaticProvider::new(&config.access_key_id, &config.secret_access_key, None);
        let client = Client::new(base_url.clone(), Some(Box::new(static_provider)), None, None).unwrap();
        let bucket_name = &config.bucket;

        // Check 'bucket_name' bucket exist or not.
        if !client.bucket_exists(&BucketExistsArgs::new(bucket_name).unwrap()).await? {
            return Err(Error::BucketNotFound(bucket_name.to_string()));
        }

        let object = client.get_object(&ObjectConditionalReadArgs::new(bucket_name, path).unwrap()).await?;
        let data = object.bytes().await.map_err(|_| Error::CantRetrieveObject(path.to_string()))?;

        Ok(data)
    }
}
