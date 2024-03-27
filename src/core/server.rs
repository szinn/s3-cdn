use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;

use crate::error::Error;

use super::{document_service::DocumentService, store_service::StoreService};

#[derive(Clone)]
pub struct Server {
    store: Arc<dyn StoreService>,
}

impl Server {
    pub fn new(store: Arc<dyn StoreService>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl DocumentService for Server {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_object(&self, path: &str) -> Result<Bytes, Error> {
        self.store.get_object(path).await
    }
}
