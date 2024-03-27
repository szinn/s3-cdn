use async_trait::async_trait;
use bytes::Bytes;

use crate::error::Error;

#[async_trait]
pub trait StoreService: Send + Sync {
    async fn get_object(&self, path: &str) -> Result<Bytes, Error>;
}
